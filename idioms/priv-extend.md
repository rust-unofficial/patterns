# `#[non_exhaustive]` for extensibility

## Description

Use `#[non_exhaustive]` on `struct`, `enum` and `enum` variant definitions to ensure that a struct is extensible without breaking
stability guarantees.

For extensive documentation on all the places `#[non_exhaustive]` can be used, see [the docs](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
## Example

```rust,ignore
mod a {
    // Public struct.
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }
    
    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VariantC { a: String }
    }
}

// Main function in another crate
fn main(s: a::S) {
    // Because S is `#[non_exhaustive]`, it cannot be named here and we must use `..`
    // in the pattern.
    let a::S { foo: _, ..} = s;
    
    let some_enum = a::AdmitMoreVariants::VariantA;
    match some_enum {
        a::AdmitMoreVariants::VariantA => println!("it's an A");
        a::AdmitMoreVariants::VariantB => println!("it's a b");
        // .. required because this variant is non-exhaustive as well
        a::AdmitMoreVariants::VariantC { a, .. } => println!("it's a c");
        // The wildcard match is required because more variants may be added in the future
        _ => println1("it's a new variant");
    }
}
```

## Discussion

Adding a field to a struct is a mostly backwards compatible change. However, if a client uses a pattern to construct or deconstruct a struct instance, they might name all the fields in the struct and adding a new one would break that pattern. The client could name some of the fields and use `..` in the pattern, in which case adding another field is backwards compatible. Rust provides `#[non_exhaustive]` to prevent clients from using code in a way that may be backwards incompatible in the future.

`#[non_exhaustive]` can also be applied to enums and their variants. A non-exhaustive enum requires that a wildcard variant must be used during matching. A `#[non_exhaustive]` variant behaves in the same way as a `#[non_exhaustive]` struct.

`#[non_exhaustive]` can make your code much less ergonomic to use, especially when forced to handle unknown enum variants. It should only be used when these sorts of evolutions are required without incrementing the major version.
