# Newtype

## Description

Use a tuple struct with a single field to make an opaque wrapper for a type.
This creates a new type, rather than an alias to a type (`type` items).

## Example

```rust
// Some type, not necessarily in the same module or even crate.
struct Foo {
    ...
}

impl Foo {
    // These functions are not present on Bar.
    ...
}

// The newtype.
pub struct Bar(Foo);

impl Bar {
    // Constructor.
    pub fn new(...) -> Bar {
        ...
    }

    ...
}

fn main() {
    let b = Bar::new(...);

    // Foo and Bar are type incompatible, the following do not type check.
    // let f: Foo = b;
    // let b: Bar = Foo { ... };
}
```


## Motivation

The primary motivation for newtypes is abstraction. It allows you to share
implementation details between types while precisely controlling the interface.
By using a newtype rather than exposing the implementation type as part of an
API, it allows you to change implementation backwards compatibly.

Newtypes can be used for distinguishing units, e.g., wrapping `f64` to give
distinguishable `Miles` and `Kms`.


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

* restricting functionality (reduce the functions exposed or traits implemented),
* making a type with copy semantics have move semantics,
* abstraction by providing a more concrete type and thus hiding internal types, e.g.,

```rust
pub struct Foo(Bar<T1, T2>);
```

Here, `Bar` might be some public, generic type and `T1` and `T2` are some internal types. Users of our module shouldn't know that we implement `Foo` by using a `Bar`, but what we're really hiding here is the types `T1` and `T2`, and how they are used with `Bar`.


## See also

[Newtypes in the style guide](http://doc.rust-lang.org/stable/style/features/types/newtype.html).

[Newtypes in Haskell](https://wiki.haskell.org/Newtype)

[Type aliases](https://doc.rust-lang.org/stable/book/type-aliases.html)
