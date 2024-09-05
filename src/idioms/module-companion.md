# Module-companion for a \[standalone\] function

## Description

In Rust, functions belong to the value [namespace], while modules belong to the type namespace. This results in a possibility of having a function and a module with the same name in the same scope [^1].

Having a module with the same name as a function can be useful for containing items (e.g. helper functions, constants, and types) that are related to that function. [^2]

Notably, it can contain

* a struct for the "parameter object" pattern,
* an error enum for the *accompanied function* [^3].

## Example

**a.rs**:

```
pub mod my_fn {
    pub enum Error {
        AlreadyExists,
        ImATeapot,
        // ...
    }

    pub struct Args {
        pub first_name: String,
        pub last_name: String,
        pub is_awesome: bool,
        pub is_lovely: bool,
        // ...
    }
}

pub fn my_fn(arg: my_fn::Args) -> Result<(), my_fn::Error> {
    // Star import from the module-companion is nearly always harmless
    use my_fn::*;

    // Destructuring the "parameter object"
    let Args {
        first_name,
        last_name,
        is_awesome,
        is_lovely,
        // ...
    } = arg;

    // ...

    Ok(())
}
```

**b.rs**:

```
// imports both the accompanied function and the module-companion `my_fn`
use crate::a::my_fn;

fn another_fn() -> anyhow::Result<()> {
    let args = my_fn::Args {
        first_name: "Amandine".to_string(),
        // TODO: change the last name
        last_name: "Cerruti".to_string(),
        is_awesome: true,
        is_lovely: true,
        // ...
    };

    my_fn(args)?;

    Ok(())
}
```

## Advantages

### Grouping related items

The *module-companion* can contain items that are related to the *accompanied function*, such as constants, helper functions, and types. This can help in organizing the codebase and making it more readable.

### Encapsulation

The *module-companion* can be used to encapsulate the implementation details of the *accompanied function*. This can help in reducing the cognitive load on the developers who are reading the code.

### Clean call sites

The *module-companion* can be used to define a "parameter object" pattern, which can help in reducing the number of arguments passed to the *accompanied function*. This can make the call sites cleaner and more readable.

## Drawbacks

### Lacking language support

The language support for this idiom is limited, so there are some rough edges that need to be worked around.
While pragmatically related, from the Rust language's perspective, the *module-companion* and the *accompanied function* are unrelated items.

### Verbosity in the function signature

Using items from the *module-companion* in the signature of the *accompanied function* requires explicitly writing the shared name.

```
pub fn my_fn(arg: my_fn::Args) -> Result<(), my_fn::Error> {
    // function body
}
```

It can have a negative impact on the readability of the function signature when the function name is long. Unfortunately, it is often the case with complex functions, which are the ones that benefit the most from this idiom.

The "workaround" for this problem - which is strictly worse - is polluting the [namespace]s of the module where they are defined with the items from the *companion module*.

### Cross-namespace name collision

There are two distinct items with the same name in the same scope but different [namespace]s:

* the *module-companion* `my_fn` in the type namespace,
* the *accompanied function* `my_fn` in the value namespace.

This can be unexpected for developers and tools.

### Module-companion for an inherent function on a type is poorly supported

While *module-companions* for standalone functions are well-supported, the same is not true for inherent functions on types (structs and enums). This is because the *module-companion* for an inherent function on a type would have to be a submodule of the type, which is not allowed in Rust.

### Rustdoc documentation could be improved

The documentation for the *module-companion* is not directly associated with the *accompanied function* in the generated documentation. This can make it harder for developers to understand the relationship between the two.

## Footnotes

[^1]: Within this article, the term "scope" - unless stated otherwise - is used loosely to refer to the collection of items (e.g. constants, structs, and functions) that belong to any of the Rust's [namespace]s and that are "visible" as a result of being defined or imported.
[^2]: Note that [procedural macros] are implemented as functions, so this idiom can be used to group the implementation details of individual procedural macros.
[^3]: For an error enum in a *companion-module*, you can consider using the [`thiserror`] crate to derive [`Error`] and [`Display`] traits. Also see the [comment about "library-like" and "application-like" errors][errors-comment] on reddit by `@dtolnay`.

[namespace]: https://doc.rust-lang.org/reference/names/namespaces.html
[procedural macros]: https://doc.rust-lang.org/reference/procedural-macros.html
[`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`thiserror`]: https://crates.io/crates/thiserror
[errors-comment]: https://www.reddit.com/r/rust/comments/dfs1zk/comment/f35iopj/
