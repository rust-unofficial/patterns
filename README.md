# Rust Design Patterns

An open source book about design patterns and idioms in the Rust programming
language that you can read [here](https://rust-unofficial.github.io/patterns/).


## Contents

[Introduction](intro.md)


### Idioms

* [Constructor](idioms/ctor.md)
* [Concatenating strings with `format!`](idioms/concat-format.md)
* [Privacy for extensibility](idioms/priv-extend.md)
* TODO stability for extensibility
* TODO trait to separate visibility of methods from visibility of data (https://github.com/sfackler/rust-postgres/blob/v0.9.6/src/lib.rs#L1400)
* [Collections are smart pointers](idioms/deref.md)
* TODO leak amplification ("Vec::drain sets the Vec's len to 0 prematurely so that mem::forgetting Drain "only" mem::forgets more stuff. instead of exposing uninitialized memory or having to update the len on every iteration")
* [Finalisation in destructors](idioms/dtor-finally.md)
* TODO interior mutability - UnsafeCell, Cell, RefCell
* [Iterating over an `Option`](idioms/option-iter.md)
* [`Default` trait](idioms/default.md)
* [Pass variables to closure](idioms/pass-var-to-closure.md)
* [`mem::replace(_)` to avoid needless clones](idioms/mem-replace.md)
* [Temporary mutability](idioms/temporary-mutability.md)
* [On-Stack Dynamic Dispatch](idioms/on-stack-dyn-dispatch.md)
* FFI usage
  - [Idiomatic Errors](idioms/ffi-errors.md)
  - [Accepting Strings](idioms/ffi-accepting-strings.md)
  - [Passing Strings](idioms/ffi-passing-strings.md)
  - TODO More Items (By being mindful of how to provide Rust libraries, and make use of existing libraries across the FFI, you can get more out of benefits Rust can bring)
* [Easy doc initialization](idioms/rustdoc-init.md)


### Design patterns

* [Builder](patterns/builder.md)
* [RAII guards](patterns/RAII.md)
* [Newtype](patterns/newtype.md)
* FFI usage
  - [Object-Based APIs](patterns/ffi-export.md)
  - [Type Consolidation into Wrappers](patterns/ffi-wrappers.md)
* TODO iterators (to safely avoid bounds checks)
* TODO closures and lifetimes (coupling to lifetime)
* TODO platform-specific sub-modules (https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md#platform-specific-opt-in)
* TODO Module organisation (by looking at examples such as Rusts `libstd`, and how it integrated into the Rusts source code, lessons can be learned about ergonomic project management and API design. Closely assosciated with platform-specific sub-modules)
* [Entry API](patterns/entry.md) (TODO Currently just a boilerplate)
* [Visitor](patterns/visitor.md)
* [Fold](patterns/fold.md)
* [Prefer small crates](patterns/small-crates.md)
* [Contain unsafety in small modules](patterns/unsafe-mods.md)
* TODO extension traits
* TODO destructor bombs (ensure linear typing dynamically, e.g., https://github.com/Munksgaard/session-types/commit/0f25ccb7c3bc9f65fa8eaf538233e8fe344a189a)
* TODO convertible to Foo trait for more generic generics (e.g., http://static.rust-lang.org/doc/master/std/fs/struct.File.html#method.open)
* [Late bound bounds](patterns/late-bounds.md) (Currently just a boilerplate)
* TODO 'shadow' borrowed version of struct - e.g., double buffering, Niko's parser generator
* TODO composition of structs to please the borrow checker
* TODO `Error` traits and `Result` forwarding
* TODO graphs
* [Compose structs together for better borrowing](patterns/compose-structs.md)


### Anti-patterns

* TODO thread + catch_panic for exceptions
* TODO Clone to satisfy the borrow checker
* [Deref polymorphism](anti_patterns/deref.md)
* TODO Matching all fields of a struct (back compat)
* TODO wildcard matches
* TODO taking an enum rather than having multiple functions
* TODO `unwrap()`ing every `Result` instead of forwarding it
* [`#[deny(warnings)]`](anti_patterns/deny-warnings.md)


## Contributing

You are missing content in this repository that can be helpful for others and you are eager to explain it?
Awesome! We are always happy about new contributions (e.g. elaboration or corrections on certain topics) to this project.

We suggest reading our [Contribution guide](./CONTRIBUTING.md) to get more information on how it works.


## Building with mdbook

This book is built with [mdbook](https://rust-lang.github.io/mdBook/). You can install it by running `cargo install mdbook`.

If you want to build it locally you can run one of these two commands in the root directory of the repository:

- `mdbook build`

  Builds static html pages as output and place them in the `/book` directory by default.

- `mdbook serve`

  Serves the book at `http://localhost:3000` (port is changeable, take a look at the terminal output 
  to be sure) and reloads the browser when a change occurs.


## License

This content of this repository is licensed under **MPL-2.0**; see [LICENSE](./LICENSE).
