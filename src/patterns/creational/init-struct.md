# Init-Struct

## Description

Construct an object relying on defaults for omitted fields.

## Example

```rust
#[derive(Debug, Default, PartialEq)]
pub struct Foo {
    pub foo: Option<u32>,
    pub bar: String,
    // Private members.
    baz: Vec<u16>
    // A lot more fields.
}


#[test]
fn init_struct_test() {
    let foo = Foo {
        bar: "Some string".to_string(),
        ..Default::default
    };
}
```

## Motivation

Useful when you have a struct with a lot of fields that have well-defined/useful defaults.

## Advantages

On complex/deeply nested structs only the fields that require explicit initalization have to be
touched.

Prevents proliferation of constructors.

Can be used for one-liner initialisation as well as more complex construction.

Avoids having to write a lot of boilerplate for the common alternative, the [Builder](builder.md)
pattern.

Can still be combined with the Builder pattern if initializtion requires transformation of some
sort.

Members can still be private as long as they implement [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html).

## Disadvantages

Requires each field of the struct to implement `Default`.

I.e. if a field is missing `Default`, `default()` has to be implemented and all fields have to be set manualy to their defaults.

## Discussion

This pattern is seen frequently in Rust since you can only have a single method with a given name.
Having multiple constructors is thus less nice in Rust than it is in C++, Java, or others.

The common way to solve this is the [Builder](builder.md) pattern but this often requires a lot of
boilerplate.

## See Also

- [Init Struct Pattern](https://xaeroxe.github.io/init-struct-pattern/)
