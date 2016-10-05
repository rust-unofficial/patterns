# Prefer small crates

## Description

Prefer small crates that do one thing well.

Cargo and crates.io make it easy to add third-party libraries, much more so than in say C or C++. We should take advantage of this tooling, and use smaller, more fine-grained dependencies.

## Advantages

* Small crates are easier to understand and audit.
* Since the compilation unit of Rust is the crate, splitting a project into multiple crates can allow more of the code to be built in parallel.
* If published on crates.io, a crate can be reused by other projects.

## Disadvantages

* Too many crates can lead to code that is hard to follow.
* This can lead to "dependency hell", where a project depends on multiple conflicting versions of a crate at the same time.
* Packages on crates.io are not curated. A crate may be poorly written, have unhelpful documentation, or be outright malicious.

## Examples

The [`ref_slice`](https://crates.io/crates/ref_slice) crate provides functions for converting `&T` to `&[T]`.

The [`url`](https://crates.io/crates/url) crate provides tools for working with URLs.

The [`num_cpus`](https://crates.io/crates/num_cpus) crate provides a function to query the number of CPUs on a machine.

## See also

* [crates.io: The Rust community crate host](https://crates.io/)
