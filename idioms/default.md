# The `Default` Trait

## Description

Many types in Rust have a [Constructor]. However, this is *specific* to the
type; Rust cannot abstract over "everything that has a `new()` method". To
allow this, the [`Default`] trait was conceived, which can be used with
containers and other generic types (e.g. see [`Option::unwrap_or_default()`]).
Notably, some containers already implement it where applicable.

Not only do one-element containers like `Cow`, `Box` or `Arc` implement
`Default` for contained `Default` types, one can automatically
`#[derive(Default)]` for structs whose fields all implement it, so the more
types implement `Default`, the more useful it becomes.

## Example

```rust
// note that we can simply auto-derive Default here.
#[derive(Default)]
struct Interesting {
    /// integers and floats default to zero
    cardinality: usize,
    /// bool defaults to false
    really: bool,
    /// collections default to empty
    see_also: Vec<Interesting>
}

fn interestingness(i: &Interesting) -> usize {
    if i.really {
        i.cardinality
    } else {
        // we can use `Option`'s `unwrap_or_default()` method here because our
        // `Interesting` struct implements `Default`.
        i.see_also.first().unwrap_or_default().cardinality
    }
}
```

## See also

- The [Constructor] idiom is another way to generate instances that may or may
not be "default"
- The [`Default`] documentation (scroll down for the list of implementors)
- [`Option::unwrap_or_default()`]

[Constructor]: ctor.md
[`Default`]: https://docs.rust-lang.org/doc/std/default/trait.Default.html
[`Option::unwrap_or_default()`]: https://docs.rust-lang.org/doc/std/option/enum.Option.html#method.unwrap_or_default
