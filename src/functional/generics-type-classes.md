# ジェネリクスを型クラスとして使用する

## 説明

Rustの型システムは、命令型言語（JavaやC++など）よりも関数型言語（Haskellなど）のように設計されています。その結果、Rustは多くの種類のプログラミング問題を「静的型付け」問題に変換できます。これは関数型言語を選択することの最大の利点の一つであり、Rustのコンパイル時保証の多くにとって重要です。

このアイデアの重要な部分は、ジェネリック型の動作方法です。例えば、C++とJavaでは、ジェネリック型はコンパイラーのメタプログラミング構成要素です。C++の`vector<int>`と`vector<char>`は、2つの異なる型で埋められた`vector`型（`template`として知られる）の同じボイラープレートコードの2つの異なるコピーに過ぎません。

Rustでは、ジェネリック型パラメーターは関数型言語で「型クラス制約」として知られているものを作成し、エンドユーザーによって埋められた各異なるパラメーターは*実際に型を変更*します。言い換えると、`Vec<isize>`と`Vec<char>`は*2つの異なる型*であり、型システムのすべての部分によって区別されるものとして認識されます。

これは**単形化**と呼ばれ、**多相**コードから異なる型が作成されます。この特別な動作により、`impl`ブロックでジェネリックパラメーターを指定する必要があります。ジェネリック型の異なる値は異なる型を引き起こし、異なる型は異なる`impl`ブロックを持つことができます。

オブジェクト指向言語では、クラスは親から動作を継承できます。しかし、これにより、型クラスの特定のメンバーに追加動作だけでなく、追加の動作も付加できます。

最も近い同等物は、JavaScriptとPythonでの実行時多相性であり、コンストラクターによって任意にオブジェクトに新しいメンバーを追加できます。しかし、これらの言語とは異なり、Rustの追加メソッドはすべて、使用時に型チェックできます。なぜなら、それらのジェネリクスは静的に定義されているからです。これにより、安全性を保ちながらより使いやすくなります。

## 例

ラボマシンの一連のストレージサーバーを設計しているとします。関連するソフトウェアのため、サポートする必要がある2つの異なるプロトコルがあります：BOOTP（PXEネットワークブート用）とNFS（リモートマウントストレージ用）です。

あなたの目標は、Rustで書かれた1つのプログラムで両方を処理することです。プロトコルハンドラーを持ち、両種類のリクエストをリッスンします。メインアプリケーションロジックは、ラボ管理者が実際のファイルのストレージとセキュリティコントロールを設定できるようにします。

ラボ内のマシンからのファイル要求は、どのプロトコルから来たかに関係なく、同じ基本情報を含んでいます：認証方法と取得するファイル名です。直接的な実装は次のようになります：

```rust,ignore
enum AuthInfo {
    Nfs(crate::nfs::AuthInfo),
    Bootp(crate::bootp::AuthInfo),
}

struct FileDownloadRequest {
    file_name: PathBuf,
    authentication: AuthInfo,
}
```

この設計は十分にうまく動作するかもしれません。しかし、今度は*プロトコル固有*のメタデータの追加をサポートする必要があるとします。例えば、NFSでは、追加のセキュリティルールを強制するために、それらのマウントポイントが何であるかを決定したいとします。

現在の構造体が設計されている方法では、プロトコルの決定は実行時まで残されます。つまり、一方のプロトコルには適用されるが他方には適用されないメソッドは、プログラマーが実行時チェックを行う必要があります。

NFSマウントポイントを取得する方法は次のようになります：

```rust,ignore
struct FileDownloadRequest {
    file_name: PathBuf,
    authentication: AuthInfo,
    mount_point: Option<PathBuf>,
}

impl FileDownloadRequest {
    // ... other methods ...

    /// これがNFSリクエストの場合、NFSマウントポイントを取得します。
    /// そうでなければ、Noneを返します。
    pub fn mount_point(&self) -> Option<&Path> {
        self.mount_point.as_ref()
    }
}
```

`mount_point()`のすべての呼び出し元は`None`をチェックし、それを処理するコードを書かなければなりません。これは、特定のコードパスで使用されるのがNFSリクエストのみであることを知っていても当てはまります！

異なるリクエストタイプが混同された場合にコンパイル時エラーを引き起こす方がはるかに最適です。結局のところ、ライブラリから使用する関数を含むユーザーのコードの全体のパスは、リクエストがNFSリクエストかBOOTPリクエストかを知っているでしょう。

Rustでは、これは実際に可能です！解決策は、APIを分割するために*ジェネリック型を追加*することです。

それは次のようになります：

```rust
use std::path::{Path, PathBuf};

mod nfs {
    #[derive(Clone)]
    pub(crate) struct AuthInfo(String); // NFS session management omitted
}

mod bootp {
    pub(crate) struct AuthInfo(); // no authentication in bootp
}

// private module, lest outside users invent their own protocol kinds!
mod proto_trait {
    use super::{bootp, nfs};
    use std::path::{Path, PathBuf};

    pub(crate) trait ProtoKind {
        type AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo;
    }

    pub struct Nfs {
        auth: nfs::AuthInfo,
        mount_point: PathBuf,
    }

    impl Nfs {
        pub(crate) fn mount_point(&self) -> &Path {
            &self.mount_point
        }
    }

    impl ProtoKind for Nfs {
        type AuthInfo = nfs::AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo {
            self.auth.clone()
        }
    }

    pub struct Bootp(); // no additional metadata

    impl ProtoKind for Bootp {
        type AuthInfo = bootp::AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo {
            bootp::AuthInfo()
        }
    }
}

use proto_trait::ProtoKind; // keep internal to prevent impls
pub use proto_trait::{Bootp, Nfs}; // re-export so callers can see them

struct FileDownloadRequest<P: ProtoKind> {
    file_name: PathBuf,
    protocol: P,
}

// all common API parts go into a generic impl block
impl<P: ProtoKind> FileDownloadRequest<P> {
    fn file_path(&self) -> &Path {
        &self.file_name
    }

    fn auth_info(&self) -> P::AuthInfo {
        self.protocol.auth_info()
    }
}

// all protocol-specific impls go into their own block
impl FileDownloadRequest<Nfs> {
    fn mount_point(&self) -> &Path {
        self.protocol.mount_point()
    }
}

fn main() {
    // your code here
}
```

このアプローチでは、ユーザーがミスを犯して間違った型を使用した場合：

```rust,ignore
fn main() {
    let mut socket = crate::bootp::listen()?;
    while let Some(request) = socket.next_request()? {
        match request.mount_point().as_ref() {
            "/secure" => socket.send("Access denied"),
            _ => {} // continue on...
        }
        // Rest of the code here
    }
}
```

構文エラーが発生します。型`FileDownloadRequest<Bootp>`は`mount_point()`を実装していません。実装しているのは型`FileDownloadRequest<Nfs>`のみです。そしてそれはもちろんNFSモジュールによって作成されるもので、BOOTPモジュールによるものではありません！

## 利点

第一に、複数の状態に共通するフィールドの重複を排除できます。共有されないフィールドをジェネリックにすることで、一度だけ実装されます。

第二に、`impl`ブロックが状態ごとに分割されるため、読みやすくなります。すべての状態に共通するメソッドは1つのブロックで一度だけ型付けされ、1つの状態に固有のメソッドは別のブロックに置かれます。

これらの両方により、コード行数が減り、より良く整理されます。

## 欠点

コンパイラーでの単形化の実装方法により、現在これはバイナリのサイズを増加させます。将来的には実装が改善される可能性があります。

## 代替案

- 型が構築や部分初期化のために「分割API」を必要とするように見える場合は、代わりに[ビルダーパターン](../patterns/creational/builder.md)を検討してください。

- 型間でAPIが変わらない場合（動作のみが変わる場合）、代わりに[ストラテジーパターン](../patterns/behavioural/strategy.md)をより良く使用できます。

## 関連項目

このパターンは標準ライブラリ全体で使用されています：

- `Vec<u8>`は、他のどんなタイプの`Vec<T>`とも異なり、Stringからキャストできます。[^1]
- イテレーターはバイナリヒープにキャストできますが、`Ord`トレイトを実装している型を含んでいる場合のみです。[^2]
- `to_string`メソッドは、`str`型の`Cow`にのみ特化されました。[^3]

また、APIの柔軟性を可能にするために、いくつかの人気のあるクレートで使用されています：

- 組み込みデバイスに使用される`embedded-hal`エコシステムは、このパターンを幅広く使用しています。例えば、組み込みピンを制御するために使用されるデバイスレジスターの設定を静的に検証することを可能にします。ピンがモードに設定されると、`Pin<MODE>`構造体を返し、そのジェネリックがそのモードで使用可能な関数を決定し、これらは`Pin`自体にはないものです。[^4]

- `hyper` HTTPクライアントライブラリは、これを使用して異なるプラグ可能リクエストに対して豊富なAPIを公開しています。異なるコネクターを持つクライアントは、異なるメソッドと異なるトレイト実装を持ち、一方でコアメソッドのセットはどのコネクターにも適用されます。[^5]

- 「タイプステート」パターン（オブジェクトが内部状態や不変条件に基づいてAPIを取得および失うパターン）は、同じ基本コンセプトと少し異なる技法を使用してRustで実装されています。[^6]

[^1]: See:
[impl From\<CString\> for Vec\<u8\>](https://doc.rust-lang.org/1.59.0/src/std/ffi/c_str.rs.html#803-811)

[^2]: See:
[impl\<T: Ord\> FromIterator\<T\> for BinaryHeap\<T\>](https://web.archive.org/web/20201030132806/https://doc.rust-lang.org/stable/src/alloc/collections/binary_heap.rs.html#1330-1335)

[^3]: See:
[impl\<'\_\> ToString for Cow\<'\_, str>](https://doc.rust-lang.org/stable/src/alloc/string.rs.html#2235-2240)

[^4]: Example:
[https://docs.rs/stm32f30x-hal/0.1.0/stm32f30x_hal/gpio/gpioa/struct.PA0.html](https://docs.rs/stm32f30x-hal/0.1.0/stm32f30x_hal/gpio/gpioa/struct.PA0.html)

[^5]: See:
[https://docs.rs/hyper/0.14.5/hyper/client/struct.Client.html](https://docs.rs/hyper/0.14.5/hyper/client/struct.Client.html)

[^6]: See:
[The Case for the Type State Pattern](https://web.archive.org/web/20210325065112/https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-the-typestate-pattern-itself/)
and
[Rusty Typestate Series (an extensive thesis)](https://web.archive.org/web/20210328164854/https://rustype.github.io/notes/notes/rust-typestate-series/rust-typestate-index)
