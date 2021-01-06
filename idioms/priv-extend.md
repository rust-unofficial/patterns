# `#[non_exhaustive]` & private fields for extensibility

## Description

A small set of scenarios exist where a library author may want to add public fields to a public struct or new variants to an enum without breaking backwards compatibility. Rust offers two solutions:
-  Use `#[non_exhaustive]` on `struct`s, `enum`s, and `enum` variants. For extensive documentation on all the places `#[non_exhaustive]` can be used, see [the docs](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
- For `struct`s only, you may add a private field to struct

**Warning**

Use this deliberately and with caution: Incrementing the major version when adding fields or variants is often a better option. `#[non_exhaustive]` may be appropriate in scenarios where you're modeling an external resource that may change out-of-sync with your library, but is not a general purpose tool.

`#[non_exhaustive]` forces clients to handle the "Something else" case; there is rarely a sensible action to take in this scenario. This leads to awkward code and code paths that only executed in extremely rare circumstances.

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

`#[non_exhaustive]` only works across crate boundaries. Within a crate, the private field method may be used:

For `struct`s, an alternative approach exists: By adding a private field to a struct, the struct cannot be instantiated or matched against.

```rust
pub struct S {
   pub a: i32,
   // Because `b` is private, you cannot match on `S` without using `..` and `S` cannot be directly instantiated
   b: ()
}
```

## Discussion

On `struct`s `#[non_exhaustive]` allows adding additional fields in a backwards compatible way. It will also prevent clients from using the struct constructor, even if all the fields are public. This may be helpful, but it's worth considering if you _want_ an additional field to be found by clients as a compiler error rather than something that may be silently undiscovered.

`#[non_exhaustive]` when applied to `enum`s forces clients to handle a wildcard variant.

Finally, #[non_exhaustive] can be applied to enum variants. A `#[non_exhaustive]` variant behaves in the same way as a `#[non_exhaustive]` struct.

### Disadvantages
`#[non_exhaustive]` can make your code much less ergonomic to use, especially when forced to handle unknown enum variants. It should only be used when these sorts of evolutions are required **without** incrementing the major version.

When `#[non_exhaustive]` is applied to `enum`s, it forces clients to handle a wildcard variant. If there is no sensible action to take in this case, this may lead to brittle code. If a client decides to `panic!()` in this scenario, it may have been better to expose this error at compile time.
