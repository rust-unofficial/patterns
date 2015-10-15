# Rust Design Patterns

An open source repository of design patterns and idioms in the Rust programming
language.


## Contents

[Introduction](intro.md)

### Idioms

* [Constructor](idioms/ctor.md)
* [Concatenating strings with `format!`](idioms/concat-format.md)
* private field to indicate extensibility
* trait to separate visibility of methods from visibility of data (https://github.com/sfackler/rust-postgres/blob/master/src/lib.rs#L1400)
* Deref on Vec/String to treat like a smart pointer/borrowed view of such data
* leak amplification ("Vec::drain sets the Vec's len to 0 prematurely so that mem::forgetting Drain "only" mem::forgets more stuff. instead of exposing uninitialized memory or having to update the len on every iteration")
* dtor for finally
* interior mutability - UnsafeCell, Cell, RefCell
* treating Option like a list

### Design patterns

* [Builder](patterns/builder.md)
* RAII ( + borrows - mutex guard)
* newtype (can be used to restrict functionality, make by-move rather than by-copy, abstraction, e.g., https://github.com/rust-lang/rust/blob/master/src/libcore/str/mod.rs#L366-L372)
* iterators (to safely avoid bounds checks)
* closures and lifetimes (coupling to lifetime)
* platform-specific sub-modules (https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md#platform-specific-opt-in)
* affine types/session types
* Entry API vs insert_or_update etc.
* visitor
* fold
* small crates and semver
* extension traits
* destructor bombs (ensure linear typing dynamically, e.g., https://github.com/Munksgaard/session-types/commit/0f25ccb7c3bc9f65fa8eaf538233e8fe344a189a)
* convertible to Foo trait for more generic generics (e.g., http://static.rust-lang.org/doc/master/std/fs/struct.File.html#method.open)
* late binding of bounds for better APIs (i.e., Mutex's don't require Send)
* 'shadow' borrowed version of struct - e.g., double buffering, Niko's parser generator
* composition of structs to please the borrow checker



### Anti-patterns

* thread + catch_panic for exceptions
* Clone to satisfy the borrow checker
* Deref polymorphism
* Matching all fields of a struct (back compat)
* wildcard matches
* tkaing an enum rather than having multiple functions



## Contributing

Contributions are very welcome!

You should start with [the template](template.md). Copy it into the appropriate
directory, edit it, and submit a PR. You might not want every section, and you
might want to add extra sections.

Correction and elaboration PRs are very welcome.
