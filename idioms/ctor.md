# Constructors

## Description

Rust does not have constructors as a language construct. Instead, the
convention is to use a static `new` method to create an object.

## Example

```rust,ignore
// A Rust vector, see liballoc/vec.rs
pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    // Constructs a new, empty `Vec<T>`.
    // Note this is a static method - no self.
    // This constructor doesn't take any arguments, but some might in order to
    // properly initialise an object
    pub fn new() -> Vec<T> {
        // Create a new Vec with fields properly initialised.
        Vec {
            // Note that here we are calling RawVec's constructor.
            buf: RawVec::new(),
            len: 0,
        }
    }
}
```

## See also

The [builder pattern](../patterns/creational/builder.md) for constructing objects
where there are multiple configurations.
