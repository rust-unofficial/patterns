# `Default`トレイト

## 説明

Rustの多くの型は[コンストラクタ][constructor]を持ちます。しかし、これはその型に*固有*です。
Rustは「`new()`メソッドを持つすべてのもの」で抽象化することはできません。これを可能にするために、
[`Default`]トレイトが考案されました。これはコンテナやその他のジェネリック型で使用できます
（例：[`Option::unwrap_or_default()`]を参照）。特に、一部のコンテナは該当する場合にすでに実装しています。

`Cow`、`Box`、`Arc`などの単一要素コンテナが含まれる`Default`型に対して`Default`を実装するだけでなく、
すべてのフィールドが実装している構造体に対して自動的に`#[derive(Default)]`を行うことができるため、
より多くの型が`Default`を実装するほど、より有用になります。

一方、コンストラクタは複数の引数を取ることができますが、`default()`メソッドはできません。
異なる名前を持つ複数のコンストラクタが存在することもありますが、一つの型につき
`Default`実装は一つだけです。

## Example

```rust
use std::{path::PathBuf, time::Duration};

// ここでは単純にDefaultを自動導出できることに注意してください。
#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    // OptionはデフォルトでNone
    output: Option<PathBuf>,
    // Vecはデフォルトで空のベクタ
    search_path: Vec<PathBuf>,
    // Durationはデフォルトでゼロ時間
    timeout: Duration,
    // boolはデフォルトでfalse
    check: bool,
}

impl MyConfiguration {
    // ここにセッターを追加
}

fn main() {
    // デフォルト値で新しいインスタンスを構築
    let mut conf = MyConfiguration::default();
    // ここでconfで何かをする
    conf.check = true;
    println!("conf = {conf:#?}");

    // デフォルト値での部分初期化、同じインスタンスを作成
    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    assert_eq!(conf, conf1);
}
```

## 関連項目

- [コンストラクタ][constructor]の慣用句は、「デフォルト」であるかどうかに関わらずインスタンスを生成する別の方法です
- [`Default`]ドキュメント（実装者のリストについてはスクロールダウン）
- [`Option::unwrap_or_default()`]
- [`derive(new)`]

[constructor]: ctor.md
[`Default`]: https://doc.rust-lang.org/stable/std/default/trait.Default.html
[`Option::unwrap_or_default()`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_default
[`derive(new)`]: https://crates.io/crates/derive-new/
