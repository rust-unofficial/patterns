# Newtype

What if in some cases we want a type to behave similar to another type or
enforce some behaviour at compile time when using only type aliases would not be
enough?

For example, if we want to create a custom `Display` implementation for `String`
due to security considerations (e.g. passwords).

For such cases we could use the `Newtype` pattern to provide **type safety** and
**encapsulation**.

## Description

Use a tuple struct with a single field to make an opaque wrapper for a type.
This creates a new type, rather than an alias to a type (`type` items).

## Example

```rust
use std::fmt::Display;

// Create Newtype Password to override the Display trait for String
struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

fn main() {
    let unsecured_password: String = "ThisIsMyPassword".to_string();
    let secured_password: Password = Password(unsecured_password.clone());
    println!("unsecured_password: {unsecured_password}");
    println!("secured_password: {secured_password}");
}
```

```shell
unsecured_password: ThisIsMyPassword
secured_password: ****************
```

## Motivation

The primary motivation for newtypes is abstraction. It allows you to share
implementation details between types while precisely controlling the interface.
By using a newtype rather than exposing the implementation type as part of an
API, it allows you to change implementation backwards compatibly.

Newtypes can be used for distinguishing units, e.g., wrapping `f64` to give
distinguishable `Miles` and `Kilometres`.

## Advantages

The wrapped and wrapper types are not type compatible (as opposed to using
`type`), so users of the newtype will never 'confuse' the wrapped and wrapper
types.

Newtypes are a zero-cost abstraction - there is no runtime overhead.

The privacy system ensures that users cannot access the wrapped type (if the
field is private, which it is by default).

## Disadvantages

The downside of newtypes (especially compared with type aliases), is that there
is no special language support. This means there can be *a lot* of boilerplate.
You need a 'pass through' method for every method you want to expose on the
wrapped type, and an impl for every trait you want to also be implemented for
the wrapper type.

## Discussion

Newtypes are very common in Rust code. Abstraction or representing units are the
most common uses, but they can be used for other reasons:

- restricting functionality (reduce the functions exposed or traits
  implemented),
- making a type with copy semantics have move semantics,
- abstraction by providing a more concrete type and thus hiding internal types,
  e.g.,

```rust,ignore
pub struct Foo(Bar<T1, T2>);
```

Here, `Bar` might be some public, generic type and `T1` and `T2` are some
internal types. Users of our module shouldn't know that we implement `Foo` by
using a `Bar`, but what we're really hiding here is the types `T1` and `T2`, and
how they are used with `Bar`.

## See also

- [Advanced Types in the book](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=newtype#using-the-newtype-pattern-for-type-safety-and-abstraction)
- [Newtypes in Haskell](https://wiki.haskell.org/Newtype)
- [Type aliases](https://doc.rust-lang.org/stable/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases)
- [derive_more](https://crates.io/crates/derive_more), a crate for deriving many
  builtin traits on newtypes.
- [The Newtype Pattern In Rust](https://web.archive.org/web/20230519162111/https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)
