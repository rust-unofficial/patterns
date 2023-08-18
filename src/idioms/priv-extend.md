# `#[non_exhaustive]` and private fields for extensibility

## Description

A small set of scenarios exist where a library author may want to add public
fields to a public struct or new variants to an enum without breaking backwards
compatibility.

Rust offers two solutions to this problem:

- Use `#[non_exhaustive]` on `struct`s, `enum`s, and `enum` variants. For
  extensive documentation on all the places where `#[non_exhaustive]` can be
  used, see
  [the docs](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).

- You may add a private field to a struct to prevent it from being directly
  instantiated or matched against (see Alternative)

## Example

```rust
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

fn print_matched_variants(s: a::S) {
    // Because S is `#[non_exhaustive]`, it cannot be named here and
    // we must use `..` in the pattern.
    let a::S { foo: _, ..} = s;
    
    let some_enum = a::AdmitMoreVariants::VariantA;
    match some_enum {
        a::AdmitMoreVariants::VariantA => println!("it's an A"),
        a::AdmitMoreVariants::VariantB => println!("it's a b"),

        // .. required because this variant is non-exhaustive as well
        a::AdmitMoreVariants::VariantC { a, .. } => println!("it's a c"),

        // The wildcard match is required because more variants may be
        // added in the future
        _ => println!("it's a new variant")
    }
}
```

## Alternative: `Private fields` for structs

`#[non_exhaustive]` only works across crate boundaries. Within a crate, the
private field method may be used.

Adding a field to a struct is a mostly backwards compatible change. However, if
a client uses a pattern to deconstruct a struct instance, they might name all
the fields in the struct and adding a new one would break that pattern. The
client could name some fields and use `..` in the pattern, in which case adding
another field is backwards compatible. Making at least one of the struct's
fields private forces clients to use the latter form of patterns, ensuring that
the struct is future-proof.

The downside of this approach is that you might need to add an otherwise
unneeded field to the struct. You can use the `()` type so that there is no
runtime overhead and prepend `_` to the field name to avoid the unused field
warning.

```rust
pub struct S {
    pub a: i32,
    // Because `b` is private, you cannot match on `S` without using `..` and `S`
    //  cannot be directly instantiated or matched against
    _b: ()
}
```

## Discussion

On `struct`s, `#[non_exhaustive]` allows adding additional fields in a backwards
compatible way. It will also prevent clients from using the struct constructor,
even if all the fields are public. This may be helpful, but it's worth
considering if you *want* an additional field to be found by clients as a
compiler error rather than something that may be silently undiscovered.

`#[non_exhaustive]` can be applied to enum variants as well. A
`#[non_exhaustive]` variant behaves in the same way as a `#[non_exhaustive]`
struct.

Use this deliberately and with caution: incrementing the major version when
adding fields or variants is often a better option. `#[non_exhaustive]` may be
appropriate in scenarios where you're modeling an external resource that may
change out-of-sync with your library, but is not a general purpose tool.

### Disadvantages

`#[non_exhaustive]` can make your code much less ergonomic to use, especially
when forced to handle unknown enum variants. It should only be used when these
sorts of evolutions are required **without** incrementing the major version.

When `#[non_exhaustive]` is applied to `enum`s, it forces clients to handle a
wildcard variant. If there is no sensible action to take in this case, this may
lead to awkward code and code paths that are only executed in extremely rare
circumstances. If a client decides to `panic!()` in this scenario, it may have
been better to expose this error at compile time. In fact, `#[non_exhaustive]`
forces clients to handle the "Something else" case; there is rarely a sensible
action to take in this scenario.

## See also

- [RFC introducing #[non_exhaustive] attribute for enums and structs](https://github.com/rust-lang/rfcs/blob/master/text/2008-non-exhaustive.md)
