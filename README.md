# Rust Design Patterns

An open source repository of design patterns and idioms in the Rust programming
language.


## Contents

[Introduction](intro.md)

### Idioms

* [Constructor](idioms/ctor.md)
* [Concatenating strings with `format!`](idioms/concat-format.md)
* TODO private field to indicate extensibility
* TODO trait to separate visibility of methods from visibility of data (https://github.com/sfackler/rust-postgres/blob/master/src/lib.rs#L1400)
* TODO Deref on Vec/String to treat like a smart pointer/borrowed view of such data
* TODO leak amplification ("Vec::drain sets the Vec's len to 0 prematurely so that mem::forgetting Drain "only" mem::forgets more stuff. instead of exposing uninitialized memory or having to update the len on every iteration")
* TODO dtor for finally
* TODO interior mutability - UnsafeCell, Cell, RefCell
* TODO treating Option like a list

### Design patterns

* [Builder](patterns/builder.md)
* TODO RAII ( + borrows - mutex guard, [style guide entry]())
* [Newtype](patterns/newtype.md)
* TODO iterators (to safely avoid bounds checks)
* TODO closures and lifetimes (coupling to lifetime)
* TODO platform-specific sub-modules (https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md#platform-specific-opt-in)
* TODO affine types/session types
* TODO Entry API vs insert_or_update etc.
* TODO visitor
* TODO fold
* TODO small crates and semver
* TODO extension traits
* TODO destructor bombs (ensure linear typing dynamically, e.g., https://github.com/Munksgaard/session-types/commit/0f25ccb7c3bc9f65fa8eaf538233e8fe344a189a)
* TODO convertible to Foo trait for more generic generics (e.g., http://static.rust-lang.org/doc/master/std/fs/struct.File.html#method.open)
* TODO late binding of bounds for better APIs (i.e., Mutex's don't require Send)
* TODO 'shadow' borrowed version of struct - e.g., double buffering, Niko's parser generator
* TODO composition of structs to please the borrow checker



### Anti-patterns

* TODO thread + catch_panic for exceptions
* TODO Clone to satisfy the borrow checker
* TODO Deref polymorphism
* TODO Matching all fields of a struct (back compat)
* TODO wildcard matches
* TODO tkaing an enum rather than having multiple functions



## Contributing

Contributions are very welcome!

You should start with [the template](template.md). Copy it into the appropriate
directory, edit it, and submit a PR. You might not want every section, and you
might want to add extra sections.

Correction and elaboration PRs are very welcome.
