# `Option`に対する反復

## 説明

`Option`は、ゼロまたは一つの要素を含むコンテナとして見ることができます。
特に、`IntoIterator`トレイトを実装しているため、そのような型を必要とする
汎用コードで使用できます。

## 例

`Option`は`IntoIterator`を実装しているため、
[`.extend()`](https://doc.rust-lang.org/std/iter/trait.Extend.html#tymethod.extend)の引数として使用できます：

```rust
let turing = Some("Turing");
let mut logicians = vec!["Curry", "Kleene", "Markov"];

logicians.extend(turing);

// 以下と同等
if let Some(turing_inner) = turing {
    logicians.push(turing_inner);
}
```

既存のイテレータの末尾に`Option`を追加する必要がある場合、
[`.chain()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)に渡すことができます：

```rust
let turing = Some("Turing");
let logicians = vec!["Curry", "Kleene", "Markov"];

for logician in logicians.iter().chain(turing.iter()) {
    println!("{logician} is a logician");
}
```

`Option`が常に`Some`である場合、代わりに要素に
[`std::iter::once`](https://doc.rust-lang.org/std/iter/fn.once.html)を使用する方が
より慣用的であることに注意してください。

また、`Option`は`IntoIterator`を実装しているため、
`for`ループを使用して反復することが可能です。これは`if let Some(..)`でマッチさせることと
同等であり、ほとんどの場合は後者を好むべきです。

## 関連項目

- [`std::iter::once`](https://doc.rust-lang.org/std/iter/fn.once.html)は
  正確に一つの要素を生成するイテレータです。`Some(foo).into_iter()`の
  より読みやすい代替案です。

- [`Iterator::filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)は
  [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)のバージョンで、
  `Option`を返すマッピング関数に特化されています。

- [`ref_slice`](https://crates.io/crates/ref_slice)クレートは、
  `Option`をゼロまたは一要素のスライスに変換する関数を提供します。

- [`Option<T>`のドキュメント](https://doc.rust-lang.org/std/option/enum.Option.html)
