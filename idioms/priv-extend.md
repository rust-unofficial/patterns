# Privacy for extensibility

## Description

Use a private field to ensure that a struct is extensible without breaking
stability guarantees.


## Example

```rust
mod a {
    // Public struct.
    pub struct S {
        pub foo: i32,
        // Private field.
        bar: i32,
    }
}

fn main(s: a::S) {
    // Because S::bar is private, it cannot be named here and we must use `..`
    // in the pattern.
    let a::S { foo: _, ..} = s;
}

```

## Discussion

Adding a field to a struct is a mostly backwards compatible change. However, if a client uses a pattern to deconstruct a struct instance, they might name all the fields in the struct and adding a new one would break that pattern. The client could name some of the fields and use `..` in the pattern, in which case adding another field is backwards compatible. Making at least one of the struct's fields private forces clients to use the latter form of patterns, ensuring that the struct is future-proof.

The downside of this approach is that you might need to add an otherwise unneeded field to the struct. You can use the `()` type so that there is no runtime overhead and prepend `_` to the field name to avoid the unused field warning.

If Rust allowed private variants of enums, we could use the same trick to make adding a variant to an enum backwards compatible. The problem there is exhaustive match expressions. A private variant would force clients to have a `_` wildcard pattern.
