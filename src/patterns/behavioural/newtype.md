# ニュータイプパターン

場合によっては、型が別の型と同じように動作することを求めたり、
型エイリアスを使用するだけでは不十分な場合にコンパイル時に特定の動作を強制したりしたいことがあります。

例えば、セキュリティ上の考慮事項（例：パスワード）のために`String`に対してカスタムの`Display`実装を作成したい場合などです。

そのような場合、**型安全性**と**カプセル化**を提供するために`Newtype`パターンを使用できます。

## 説明

単一のフィールドを持つタプル構造体を使用して、型の不透明なラッパーを作成します。
これは、型へのエイリアス（`type`項目）ではなく、新しい型を作成します。

## Example

```rust
use std::fmt::Display;

// Create Newtype Password to override the Display trait for String
struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

fn main() {
    let unsecured_password: String = "ThisIsMyPassword".to_string();
    let secured_password: Password = Password(unsecured_password.clone());
    println!("unsecured_password: {unsecured_password}");
    println!("secured_password: {secured_password}");
}
```

```shell
unsecured_password: ThisIsMyPassword
secured_password: ****************
```

## 動機

ニュータイプの主要な動機は抽象化です。インターフェースを正確に制御しながら、
型間で実装の詳細を共有することができます。
APIの一部として実装型を公開するのではなく、ニュータイプを使用することで、
後方互換性を保って実装を変更できます。

ニュータイプは単位の区別に使用できます。例えば、`f64`をラップして
区別可能な`Miles`と`Kilometres`を提供することができます。

## 利点

ラップされた型とラッパー型は型互換性がない（`type`を使用するのとは対照的に）ため、
ニュータイプのユーザーはラップされた型とラッパー型を「混同」することはありません。

ニュータイプはゼロコスト抽象化です - 実行時のオーバーヘッドはありません。

プライバシーシステムにより、ユーザーはラップされた型にアクセスできません
（フィールドがプライベートの場合、デフォルトでそうなります）。

## 欠点

ニュータイプの欠点（特に型エイリアスと比較して）は、特別な言語サポートがないことです。
これは*多くの*ボイラープレートが存在する可能性があることを意味します。
ラップされた型で公開したいすべてのメソッドに対して「パススルー」メソッドが必要であり、
ラッパー型でも実装したいすべてのトレイトに対してimplが必要です。

## 議論

ニュータイプはRustコードで非常に一般的です。抽象化や単位の表現が最も一般的な用途ですが、
他の理由でも使用できます：

- 機能の制限（公開される関数や実装されるトレイトを減らす）
- コピーセマンティクスを持つ型にムーブセマンティクスを持たせる
- より具体的な型を提供して内部型を隠すことによる抽象化、例：

```rust,ignore
pub struct Foo(Bar<T1, T2>);
```

ここで、`Bar`は何らかのパブリックなジェネリック型であり、
`T1`と`T2`は内部型です。私たちのモジュールのユーザーは、
`Bar`を使用して`Foo`を実装していることを知る必要はありませんが、
ここで実際に隠しているのは型`T1`と`T2`、
そしてそれらが`Bar`でどのように使用されるかです。

## 参照

- [書籍の高度な型](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=newtype#using-the-newtype-pattern-for-type-safety-and-abstraction)
- [HaskellのNewtypes](https://wiki.haskell.org/Newtype)
- [型エイリアス](https://doc.rust-lang.org/stable/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases)
- [derive_more](https://crates.io/crates/derive_more)、ニュータイプに多くの組み込みトレイトを派生するためのクレート。
- [RustのNewtype Pattern](https://web.archive.org/web/20230519162111/https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)
