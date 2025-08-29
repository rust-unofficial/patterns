# 文字列を渡す

## 説明

FFI関数に文字列を渡す際、従うべき4つの原則があります：

1. 所有された文字列のライフタイムを可能な限り長くする。
2. 変換中の`unsafe`コードを最小化する。
3. Cコードが文字列データを変更できる場合、`CString`の代わりに`Vec`を使用する。
4. 外部関数APIが要求しない限り、文字列の所有権は呼び出し先に譲渡すべきではない。

## 動機

Rustは`CString`と`CStr`型でC形式の文字列のサポートを組み込んでいます。
しかし、Rust関数から外部関数呼び出しに送信される文字列について取れる
異なるアプローチがあります。

ベストプラクティスは簡単です：`unsafe`コードを最小化するような方法で
`CString`を使用する。しかし、二次的な注意点は*オブジェクトが十分長く生きなければならない*
ことであり、ライフタイムを最大化すべきことを意味します。さらに、ドキュメントでは
変更後の`CString`の「往復」はUBであると説明しているため、
その場合は追加の作業が必要です。

## Code Example

```rust,ignore
pub mod unsafe_module {

    // other module content

    extern "C" {
        fn seterr(message: *const libc::c_char);
        fn geterr(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    }

    fn report_error_to_ffi<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
        let c_err = std::ffi::CString::new(err.into())?;

        unsafe {
            // SAFETY: ドキュメントでポインターがconstであると述べている
            // FFIを呼び出すため、変更は発生しないはず
            seterr(c_err.as_ptr());
        }

        Ok(())
        // c_errのライフタイムはここまで続く
    }

    fn get_error_from_ffi() -> Result<String, std::ffi::IntoStringError> {
        let mut buffer = vec![0u8; 1024];
        unsafe {
            // SAFETY: ドキュメントが入力が呼び出しと同じ期間だけ
            // 生きていればよいことを示唆するFFIを呼び出し
            let written: usize = geterr(buffer.as_mut_ptr(), 1023).into();

            buffer.truncate(written + 1);
        }

        std::ffi::CString::new(buffer).unwrap().into_string()
    }
}
```

## 利点

この例は以下を確保するような方法で書かれています：

1. `unsafe`ブロックが可能な限り小さい。
2. `CString`が十分長く生きる。
3. 型キャストでのエラーが可能な場合常に伝播される。

一般的な間違い（ドキュメントに載るほど一般的）は、最初のブロックで
変数を使用しないことです：

```rust,ignore
pub mod unsafe_module {

    // other module content

    fn report_error<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
        unsafe {
            // SAFETY: おっと、これはダングリングポインターを含んでいます！
            seterr(std::ffi::CString::new(err.into())?.as_ptr());
        }
        Ok(())
    }
}
```

このコードはダングリングポインターを生成します。なぜなら、参照が作成された場合とは異なり、
`CString`のライフタイムはポインター作成によって延長されないからです。

よく提起されるもう一つの問題は、1kのゼロのベクタの初期化が「遅い」ということです。
しかし、Rustの最近のバージョンでは実際にその特定のマクロを`zmalloc`への呼び出しに
最適化しており、これはオペレーティングシステムがゼロ化されたメモリを返す能力と同じ速さ
（かなり高速）であることを意味します。

## 欠点

なし？
