# ビルダー

## 説明

ビルダーヘルパーへの呼び出しでオブジェクトを構築します。

## 例

```rust
#[derive(Debug, PartialEq)]
pub struct Foo {
    // 多くの複雑なフィールド
    bar: String,
}

impl Foo {
    // このメソッドはユーザーがビルダーを発見するのに役立ちます
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    // おそらく多くのオプショナルなフィールド
    bar: String,
}

impl FooBuilder {
    pub fn new(/* ... */) -> FooBuilder {
        // Fooの最小限必要なフィールドを設定
        FooBuilder {
            bar: String::from("X"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        // ビルダー自体に名前を設定し、ビルダーを値で返す
        self.bar = bar;
        self
    }

    // ここでビルダーを消費しないで済むなら、それは利点です。
    // これは、FooBuilderを多くのFooを構築するためのテンプレートとして
    // 使用できることを意味します。
    pub fn build(self) -> Foo {
        // FooBuilderからFooを作成し、FooBuilderのすべての設定を
        // Fooに適用します。
        Foo { bar: self.bar }
    }
}

#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();
    assert_eq!(foo, foo_from_builder);
}
```

## 動機

多くのコンストラクタが必要になる場合や、構築に副作用がある場合に有用です。

## 利点

構築用のメソッドを他のメソッドから分離します。

コンストラクタの増殖を防ぎます。

ワンライナーの初期化にも、より複雑な構築にも使用できます。

## 欠点

構造体オブジェクトを直接作成したり、単純なコンストラクタ関数を使用したりするよりも複雑です。

## 議論

このパターンは、Rustにはオーバーロードがないため、他の多くの言語よりもRustで（そしてより単純なオブジェクトに対して）頻繁に見られます。
指定された名前のメソッドは1つしか持てないため、複数のコンストラクタを持つことは、
C++、Java、その他の言語よりもRustでは好ましくありません。

このパターンは、ビルダーオブジェクトが単なるビルダーではなく、それ自体が有用である場合によく使用されます。
例えば、[`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html)は
[`Child`](https://doc.rust-lang.org/std/process/struct.Child.html)（プロセス）のビルダーです。
これらの場合、`T`と`TBuilder`の命名パターンは使用されません。

この例では、ビルダーを値で取得して返します。多くの場合、ビルダーを可変参照として取得して返す方が、
よりエルゴノミック（そしてより効率的）です。借用チェッカーはこれを自然に機能させます。
このアプローチには、次のようなコードを書けるという利点があります：

```rust,ignore
let mut fb = FooBuilder::new();
fb.a();
fb.b();
let f = fb.build();
```

`FooBuilder::new().a().b().build()`スタイルと同様です。

## 参照

- [スタイルガイドの説明](https://web.archive.org/web/20210104103100/https://doc.rust-lang.org/1.12.0/style/ownership/builders.html)
- [derive_builder](https://crates.io/crates/derive_builder)、ボイラープレートを避けながら
  このパターンを自動的に実装するクレート
- [コンストラクタパターン](../../idioms/ctor.md) - より単純な構築の場合
- [ビルダーパターン (wikipedia)](https://en.wikipedia.org/wiki/Builder_pattern)
- [複雑な値の構築](https://web.archive.org/web/20210104103000/https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder)