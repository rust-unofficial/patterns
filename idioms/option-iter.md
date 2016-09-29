# Iterating over an `Option`

## Description

`Option` can be viewed as a container that contains either zero or one elements. In particular, it implements the `IntoIterator` trait, and as such can be used with generic code that needs such a type.

## Examples

Since `Option` implements `IntoIterator`, it can be used as an argument to [`.extend()`](https://doc.rust-lang.org/std/iter/trait.Extend.html#tymethod.extend):

```rust
let turing = Some("Turing");
let mut logicians = vec!["Curry", "Kleene", "Markov"];

logicians.extend(turing);

// equivalent to
if let Some(turing_inner) = turing {
    bar.push(turing_inner);
}
```

If you need to tack an `Option` to the end of an existing iterator, you can pass it to [`.chain()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain):

```rust
let turing = Some("Turing");
let logicians = vec!["Curry", "Kleene", "Markov"];

for logician in logicians.iter().chain(turing.iter()) {
    println!("{} is a logician", logician);
}
```

Note that if the `Option` is always `Some`, then it is more idiomatic to use [`std::iter::once`](https://doc.rust-lang.org/std/iter/fn.once.html) on the element instead.

Also, since `Option` implements `IntoIterator`, it's possible to iterate over it using a `for` loop. This is equivalent to matching it with `if let Some(..)`, and in most cases you should prefer the latter.

## See also

* [`std::iter::once`](https://doc.rust-lang.org/std/iter/fn.once.html) is an iterator which yields exactly one element. It's a more readable alternative to `Some(foo).into_iter()`.

* [`Iterator::filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) is a version of [`Iterator::flat_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map), specialized to mapping functions which return `Option`.

* The [`ref_slice`](https://crates.io/crates/ref_slice) crate provides functions for converting an `Option` to a zero- or one-element slice.

* [Documentation for `Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html)
