# 文字列を受け入れる

## 説明

FFIを介してポインターで文字列を受け入れる際、従うべき2つの原則があります：

1. 外部の文字列を直接コピーするのではなく、「借用」状態に保つ。
2. C形式の文字列をネイティブRust文字列に変換する際の複雑さと
   `unsafe`コードの量を最小化する。

## 動機

Cで使用される文字列は、Rustで使用されるものとは異なる動作を持ちます：

- C文字列はnull終端であるのに対し、Rust文字列は長さを格納します
- C文字列は任意の非ゼロバイトを含むことができるのに対し、Rust文字列は
  UTF-8でなければなりません
- C文字列は`unsafe`ポインタ操作を使用してアクセス・操作されるのに対し、
  Rust文字列との相互作用は安全なメソッドを通じて行われます

Rust標準ライブラリには、RustのC文字列等価物である`CString`と`&CStr`があり、
これらはC文字列とRust文字列間の変換に関わる多くの複雑さと
`unsafe`コードを避けることを可能にします。

`&CStr`型は借用データで作業することも可能にし、RustとC間で文字列を
渡すことがゼロコスト操作であることを意味します。

## Code Example

```rust,ignore
pub mod unsafe_module {

    // other module content

    /// 指定されたレベルでメッセージをログ出力します。
    ///
    /// # Safety
    ///
    /// `msg`が以下を満たすことは呼び出し元の保証です：
    ///
    /// - nullポインターではない
    /// - 有効で初期化されたデータを指す
    /// - nullバイトで終了するメモリを指す
    /// - この関数呼び出しの期間中変更されない
    #[no_mangle]
    pub unsafe extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
        let level: crate::LogLevel = match level { /* ... */ };

        // SAFETY: The caller has already guaranteed this is okay (see the
        // `# Safety` section of the doc-comment).
        let msg_str: &str = match std::ffi::CStr::from_ptr(msg).to_str() {
            Ok(s) => s,
            Err(e) => {
                crate::log_error("FFI string conversion failed");
                return;
            }
        };

        crate::log(msg_str, level);
    }
}
```

## 利点

この例は以下を確保するように書かれています：

1. `unsafe`ブロックが可能な限り小さい。
2. 「追跡されていない」ライフタイムを持つポインターが「追跡される」共有参照になる

文字列が実際にコピーされる代替案を考えてみましょう：

```rust,ignore
pub mod unsafe_module {

    // other module content

    pub extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
        // このコードは使用しないでください。
        // 醜く、冗長で、微妙なバグが含まれています。

        let level: crate::LogLevel = match level { /* ... */ };

        let msg_len = unsafe { /* SAFETY: strlenはそれが何であるかですよね？ */
            libc::strlen(msg)
        };

        let mut msg_data = Vec::with_capacity(msg_len + 1);

        let msg_cstr: std::ffi::CString = unsafe {
            // SAFETY: スタックフレーム全体で生存すると期待される
            // 外部ポインターから所有メモリにコピー
            std::ptr::copy_nonoverlapping(msg, msg_data.as_mut(), msg_len);

            msg_data.set_len(msg_len + 1);

            std::ffi::CString::from_vec_with_nul(msg_data).unwrap()
        }

        let msg_str: String = unsafe {
            match msg_cstr.into_string() {
                Ok(s) => s,
                Err(e) => {
                    crate::log_error("FFI string conversion failed");
                    return;
                }
            }
        };

        crate::log(&msg_str, level);
    }
}
```

このコードは元のコードに対して2つの点で劣っています：

1. `unsafe`コードがはるかに多く、より重要なことに、維持しなければならない
   不変条件が多い。
2. 必要な広範囲な算術演算のため、このバージョンにはRustの
   `未定義動作`を引き起こすバグがあります。

ここでのバグは、ポインタ算術での単純な間違いです：文字列はコピーされ、
その`msg_len`バイトすべてがコピーされました。しかし、末尾の`NUL`終端子は
コピーされませんでした。

ベクタのサイズは*ゼロパディングされた文字列*の長さに*設定*されました --
末尾にゼロを追加できた*リサイズ*ではなく。結果として、ベクタの最後のバイトは
初期化されていないメモリです。ブロックの下部で`CString`が作成される際、
そのベクタの読み取りは`未定義動作`を引き起こします！

多くのこのような問題と同様に、これは追跡が困難な問題でしょう。文字列が
`UTF-8`でないためにパニックすることもあれば、文字列の末尾に奇妙な
文字を置くこともあり、完全にクラッシュすることもあります。

## 欠点

なし？
