# The `Default` Trait

## Description

Many types in Rust have a [constructor]. However, this is *specific* to the
type; Rust cannot abstract over "everything that has a `new()` method". To
allow this, the [`Default`] trait was conceived, which can be used with
containers and other generic types (e.g. see [`Option::unwrap_or_default()`]).
Notably, some containers already implement it where applicable.

Not only do one-element containers like `Cow`, `Box` or `Arc` implement
`Default` for contained `Default` types, one can automatically
`#[derive(Default)]` for structs whose fields all implement it, so the more
types implement `Default`, the more useful it becomes.

On the other hand, constructors can take multiple arguments, while the
`default()` method does not. There can even be multiple constructors with
different names, but there can only be one `Default` implementation per type.

## Example

```rust
// note that we can simply auto-derive Default here.
#[derive(Default)]
struct MyConfiguration {
    // Option defaults to None
    output: Option<Path>,
    // Vecs default to empty vector
    search_path: Vec<Path>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}

impl MyConfiguration {
    // add setters here
}
```

## See also

- The [constructor] idiom is another way to generate instances that may or may
not be "default"
- The [`Default`] documentation (scroll down for the list of implementors)
- [`Option::unwrap_or_default()`]
- [`derive(new)`]

[constructor]: ctor.md
[`Default`]: https://docs.rust-lang.org/doc/std/default/trait.Default.html
[`Option::unwrap_or_default()`]: https://docs.rust-lang.org/doc/std/option/enum.Option.html#method.unwrap_or_default
[`derive(new)`]: https://crates.io/crates/derive-new/
