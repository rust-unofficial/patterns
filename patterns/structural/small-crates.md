# Prefer small crates

## Description

Prefer small crates that do one thing well.

Cargo and crates.io make it easy to add third-party libraries, much more so than
in say C or C++. Moreover, since packages on crates.io cannot be edited or removed
after publication, any build that works now should continue to work in the future.
We should take advantage of this tooling, and use smaller, more fine-grained dependencies.

## Advantages

* Small crates are easier to understand, and encourage more modular code.
* Crates allow for re-using code between projects.
  For example, the `url` crate was developed as part of the Servo browser engine,
  but has since found wide use outside the project.
* Since the compilation unit
  of Rust is the crate, splitting a project into multiple crates can allow more of
  the code to be built in parallel.

## Disadvantages

* This can lead to "dependency hell", when a project depends on multiple conflicting
  versions of a crate at the same time. For example, the `url` crate has both versions
  1.0 and 0.5. Since the `Url` from `url:1.0` and the `Url` from `url:0.5` are
  different types, an HTTP client that uses `url:0.5` would not accept `Url` values
  from a web scraper that uses `url:1.0`.
* Packages on crates.io are not curated. A crate may be poorly written, have
  unhelpful documentation, or be outright malicious.
* Two small crates may be less optimized than one large one, since the compiler
  does not perform link-time optimization (LTO) by default.

## Examples

The [`ref_slice`](https://crates.io/crates/ref_slice) crate provides functions
for converting `&T` to `&[T]`.

The [`url`](https://crates.io/crates/url) crate provides tools for working with
URLs.

The [`num_cpus`](https://crates.io/crates/num_cpus) crate provides a function to
query the number of CPUs on a machine.

## See also

* [crates.io: The Rust community crate host](https://crates.io/)
