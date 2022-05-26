# `#![deny(warnings)]`

## Description

A well-intentioned crate author wants to ensure their code builds without
warnings. So they annotate their crate root with the following:

## Example

```rust
#![deny(warnings)]

// All is well.
```

## Advantages

It is short and will stop the build if anything is amiss.

## Drawbacks

By disallowing the compiler to build with warnings, a crate author opts out of
Rust's famed stability. Sometimes new features or old misfeatures need a change
in how things are done, thus lints are written that `warn` for a certain grace
period before being turned to `deny`.

For example, it was discovered that a type could have two `impl`s with the same
method. This was deemed a bad idea, but in order to make the transition smooth,
the `overlapping-inherent-impls` lint was introduced to give a warning to those
stumbling on this fact, before it becomes a hard error in a future release.

Also sometimes APIs get deprecated, so their use will emit a warning where
before there was none.

All this conspires to potentially break the build whenever something changes.

Furthermore, crates that supply additional lints (e.g. [rust-clippy]) can no
longer be used unless the annotation is removed. This is mitigated with
[--cap-lints]. The `--cap-lints=warn` command line argument, turns all `deny`
lint errors into warnings.

## Alternatives

There are two ways of tackling this problem: First, we can decouple the build
setting from the code, and second, we can name the lints we want to deny
explicitly.

The following command line will build with all warnings set to `deny`:

```RUSTFLAGS="-D warnings" cargo build```

This can be done by any individual developer (or be set in a CI tool like
Travis, but remember that this may break the build when something changes)
without requiring a change to the code.

Alternatively, we can specify the lints that we want to `deny` in the code.
Here is a list of warning lints that is (hopefully) safe to deny (as of Rustc 1.48.0):

```rust,ignore
#![deny(bad_style,
       const_err,
       dead_code,
       improper_ctypes,
       non_shorthand_field_patterns,
       no_mangle_generic_items,
       overflowing_literals,
       path_statements,
       patterns_in_fns_without_body,
       private_in_public,
       unconditional_recursion,
       unused,
       unused_allocation,
       unused_comparisons,
       unused_parens,
       while_true)]
```

In addition, the following `allow`ed lints may be a good idea to `deny`:

```rust,ignore
#![deny(missing_debug_implementations,
       missing_docs,
       trivial_casts,
       trivial_numeric_casts,
       unused_extern_crates,
       unused_import_braces,
       unused_qualifications,
       unused_results)]
```

Some may also want to add `missing-copy-implementations` to their list.

Note that we explicitly did not add the `deprecated` lint, as it is fairly
certain that there will be more deprecated APIs in the future.

## See also

- [A collection of all clippy lints](https://rust-lang.github.io/rust-clippy/master)
- [deprecate attribute] documentation
- Type `rustc -W help` for a list of lints on your system. Also type
`rustc --help` for a general list of options
- [rust-clippy] is a collection of lints for better Rust code

[rust-clippy]: https://github.com/Manishearth/rust-clippy
[deprecate attribute]: https://doc.rust-lang.org/reference/attributes.html#deprecation
[--cap-lints]: https://doc.rust-lang.org/rustc/lints/levels.html#capping-lints
