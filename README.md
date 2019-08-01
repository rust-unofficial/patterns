# Rust Design Patterns

An open source repository of design patterns and idioms in the Rust programming
language.


## Contents

[Introduction](intro.md)

### Idioms

* [Constructor](idioms/ctor.md)
* [Concatenating strings with `format!`](idioms/concat-format.md)
* [Privacy for extensibility](idioms/priv-extend.md)
* TODO stability for extensibility
* TODO trait to separate visibility of methods from visibility of data (https://github.com/sfackler/rust-postgres/blob/master/src/lib.rs#L1400)
* [Collections are smart pointers](idioms/deref.md)
* TODO leak amplification ("Vec::drain sets the Vec's len to 0 prematurely so that mem::forgetting Drain "only" mem::forgets more stuff. instead of exposing uninitialized memory or having to update the len on every iteration")
* [Finalisation in destructors](idioms/dtor-finally.md)
* TODO interior mutability - UnsafeCell, Cell, RefCell
* [Iterating over an `Option`](idioms/option-iter.md)
* [`Default` trait](idioms/default.md)
* [Pass variables to closure](idioms/pass-var-to-closure.md)
* [`mem::replace(_)` to avoid needless clones](idioms/mem-replace.md)
* [Temporary mutability](idioms/temporary-mutability.md)

### Design patterns

* [Builder](patterns/builder.md)
* [RAII guards](patterns/RAII.md)
* [Newtype](patterns/newtype.md)
* TODO iterators (to safely avoid bounds checks)
* TODO closures and lifetimes (coupling to lifetime)
* TODO platform-specific sub-modules (https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md#platform-specific-opt-in)
* [Entry API](patterns/entry.md) (Currently just a boilerplate)
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

Contributions are very welcome!

You should start with [the template](template.md). Copy it into the appropriate
directory, edit it, and submit a PR. You might not want every section, and you
might want to add extra sections.

We suggest leaving a comment on the [issue tracker](https://github.com/rust-unofficial/patterns/issues)
so that other people don't start working on the same topic.

Correction and elaboration PRs are very welcome.
