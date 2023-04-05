# Constructors

## Description

Rust does not have constructors as a language construct. Instead, the
convention is to use an [associated function][] `new` to create an object:

```rust
/// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::new(42);
/// assert_eq!(42, s.value());
/// ```
pub struct Second {
    value: u64
}

impl Second {
    // Constructs a new instance of [`Second`].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}
```

## Default Constructors

Rust supports default constructors with the [`Default`][std-default] trait:

```rust
/// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
pub struct Second {
    value: u64
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}
```

`Default` can also be derived if all types of all fields implement `Default`,
like they do with `Second`:

```rust
/// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
#[derive(Default)]
pub struct Second {
    value: u64
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}
```

**Note:** It is common and expected for types to implement both
`Default` and an empty `new` constructor. `new` is the constructor
convention in Rust, and users expect it to exist, so if it is
reasonable for the basic constructor to take no arguments, then it
should, even if it is functionally identical to default.

**Hint:** The advantage of implementing or deriving `Default` is that your type
can now be used where a `Default` implementation is required, most prominently,
any of the [`*or_default` functions in the standard library][std-or-default].

## See also

- The [default idiom](default.md) for a more in-depth description of the
  `Default` trait.

- The [builder pattern](../patterns/creational/builder.md) for constructing
  objects where there are multiple configurations.

- [API Guidelines/C-COMMON-TRAITS][API Guidelines/C-COMMON-TRAITS] for
  implementing both, `Default` and `new`.

[associated function]: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#associated-functions
[std-default]: https://doc.rust-lang.org/stable/std/default/trait.Default.html
[std-or-default]: https://doc.rust-lang.org/stable/std/?search=or_default
[API Guidelines/C-COMMON-TRAITS]: https://rust-lang.github.io/api-guidelines/interoperability.html#types-eagerly-implement-common-traits-c-common-traits
