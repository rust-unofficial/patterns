# Sealed Trait

## Description

In Rust, a sealed trait is a trait that can only be implemented within the same
crate where it is defined. It is achieved by making a public trait that depends
on a [supertrait](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html)
that is private or only public on the local crate. This restricts the ability
to implement the trait from outside the crate and provides a form of interface
segregation.

## Motivation

Sealed traits can be used to create a set of behaviors that that allow future
addition of methods without breaking the API.

The sealed trait pattern helps to separate the public interface from the
implementation details. This makes it easier to maintain and evolve the
implementation over time without breaking existing code. It also provides a
level of abstraction that allows users to interact with the library or module at
a high level, without needing to know the implementation details.

Because users of this crate can't implement this trait directly, is possible to
add methods to it, without break existing code using this create. Only
implementations of this trait will need updated, what is assured by the
sealed trait to only happen locally.

## Example

One possible use of the sealed trait is to limit what kind of implementation a
function can receive, allowing only a limited number of types to be passed as
parameters.

```rust,ignore
pub(crate) mod private {
    pub(crate) trait Sealed {}
}
// MyStruct is Sealed, and only this crate have access to it. Other crates will
// be able to implement it.
pub trait MyStruct: private::Sealed {...}
// auto implement Sealed for any type that implement MyStruct
impl<T: MyStruct> private::Sealed for T {}

pub struct MyStructA {...}
impl MyStruct for MyStructA {...}

pub struct MyStructB {...}
impl MyStruct for MyStructB {...}

// this function will only receive MyStructA or MyStructB because they are the
// only ones that implement the MyStruct trait
pub fn receive_my_struct(my_struct: impl MyStruct) {...}
```

The standard library makes use of a sealed trait, one example is the
`OsStrExt` trait for
[unix](https://doc.rust-lang.org/std/os/unix/ffi/trait.OsStrExt.html),
[windows](https://doc.rust-lang.org/std/os/windows/ffi/trait.OsStrExt.html) and
[wasi](https://doc.rust-lang.org/std/os/wasi/ffi/trait.OsStrExt.html).

Trait from `std::os::unix::ffi::OsStrExt`:

```rust,ignore
pub trait OsStrExt: Sealed {
    fn from_bytes(slice: &[u8]) -> &Self;
    fn as_bytes(&self) -> &[u8];
}
```

The `Sealed` trait is private and cannot be accessed from outside the standard
library. Not allowing users to implement `OsStrExt` for any type, except for the
implementations already present on the standard library.

The documentation describes it's motivation as the following:

> This trait is sealed: it cannot be implemented outside the standard library.
This is so that future additional methods are not breaking changes.

## Advantages

By separating the public interface from the implementation details, it is
easier to maintain, ensure that the code remains correct without affecting
external users.

## Disadvantages

Although it usually reduces complexity and code duplication, the sealed trait
pattern can add complexity to the codebase, particularly if there are many
sealed traits that need to be managed.

## Discussion

Sealed traits are a useful tool for creating a set of related behaviors that are
intended to be used together without allowing other behaviors to be added from
outside the crate.

This restriction also allow future additions to the trait
without compromising the compatibility with existing code uses of it.

## See also

Blog post from
[Predrag](https://web.archive.org/web/20230406211349/https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/)
about sealed, private and other pattern for traits.
