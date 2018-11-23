# Contain Unsafety in Small Modules

## Description

If you have `unsafe` code, create the smallest possible module that can uphold the needed invariants to build a minimal safe interface upon the unsafety. Embed this into a larger module that contains only safe code and presents an ergonomic interface. Note that the outer module can contain unsafe functions and methods that call directly into the unsafe code. Users may use this to gain speed benefits.

## Advantages

* This restricts the unsafe code that must be audited
* Writing the outer module is much easier, since you can count on the guarantees of the inner module

## Disadvantages

* Sometimes, it may be hard to find a suitable interface.
* The abstraction may introduce inefficiencies.

## Examples

* The [`optional`](https://crates.io/crates/optional) crate has one unsafe module for converting a `&T` to either a slice of one `[T]` or an empty slice.
* `std`s `String` class is basically just a wrapper over `Vec<u8>` with the added invariant that the contents must be valid unicode. The operations on `String` ensure this behavior, however, users have the option of using an `unsafe` method to create a `String`, in which case the onus is on them to guarantee the validity of the contents.

## See also

* [Ralf Jung's Blog about invariants in unsafe code](https://www.ralfj.de/blog/2018/08/22/two-kinds-of-invariants.html)
