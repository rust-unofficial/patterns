# Prefer small crates

## Description

Prefer small crates that do one thing well.

Cargo and crates.io make it easy to add third-party libraries, much more so than
in say C or C++. Moreover, since packages on crates.io cannot be edited or
removed after publication, any build that works now should continue to work in
the future. We should take advantage of this tooling, and use smaller, more
fine-grained dependencies.

## Advantages

- Small crates are easier to understand, and encourage more modular code.
- Crates allow for re-using code between projects. For example, the `url` crate
  was developed as part of the Servo browser engine, but has since found wide
  use outside the project.
- Since the compilation unit of Rust is the crate, splitting a project into
  multiple crates can allow more of the code to be built in parallel.

## Disadvantages

- This can lead to "dependency hell", when a project depends on multiple
  conflicting versions of a crate at the same time. For example, the `url` crate
  has both versions 1.0 and 0.5. Since the `Url` from `url:1.0` and the `Url`
  from `url:0.5` are different types, an HTTP client that uses `url:0.5` would
  not accept `Url` values from a web scraper that uses `url:1.0`.
  Rust and Cargo provide several options to handle this disadvantage:
  - [workspace.dependencies](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table)
    helps in enforcing the same crate version across all workspace members
  - dependnecy can be renamed in cargo.toml and in Rust module import 
  - `package.resolver= "2"` allows to have several indirect and direct dependncies to be in same workspace,
     so sometimes it fails (for few reasons), one can:
    - if resolver fails to find compatible versions, can use [patch](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section)
      to pin dependecy accross `Cargo.lock`
    - `resolver = "3"` added Rust-version awareness,
       so incompatible crate versions (based on the `rust-version` field) are not selected,
       further reducing potential dependency hell
   - [zepter](https://github.com/ggwpez/zepter) can be used to resolve feature resolution conflicts
      
- Packages on [crates.io](https://crates.io) are not curated. A crate may be poorly written, have
  unhelpful documentation, or be outright malicious.
  Rust ecosystem has next tools to mitigate these disadvantages:
  - [`cargo-audit`](https://github.com/rustsec/rustsec),
    [`cargo-vet`](https://github.com/mozilla/cargo-vet),
    [`cargo-crev`](https://github.com/crev-dev/cargo-crev)
    and [`cargo-geiger`](https://github.com/rust-secure-code/cargo-geiger)
    can be used to check audit status of dependencies
  - [`OCI images`](https://opencontainers.org/), [`nix`](https://nixos.org/) and `nix` can be used for lightweight sandbox builds, to prevent `build.rs` and `proc_macro` to access IO.
  - LLMs can be used for providing documentation and general assessment of dependency

- Two small crates may be less optimized than one large one, since the compiler
  does not [perform link-time optimization (LTO)](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) by default.

## Examples

The [`url`](https://crates.io/crates/url) crate provides tools for working with
URLs.

The [`num_cpus`](https://crates.io/crates/num_cpus) crate provides a function to
query the number of CPUs on a machine.

The [`ref_slice`](https://crates.io/crates/ref_slice) crate provides functions
for converting `&T` to `&[T]`. (Historical example)

## See also

- [crates.io: The Rust community crate host](https://crates.io/)
