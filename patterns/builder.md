# Builder

## Description

Construct an object with calls to a builder helper. 


## Example

```rust
struct Foo {
    // Lots of complicated fields.
}

struct FooBuilder {
    // Probably lots of optional fields.
    ...
}

impl FooBuilder {
    fn new(...) -> FooBuilder {
        // Set the minimally required fields of Foo.
    }

    fn named(mut self, name: &str) -> FooBuilder {
        // Set the name on the builder itself, and return the builder by value.
    }

    // More methods that take `mut self` and return `FooBuilder` setting up
    // various aspects of a Foo.
    ...

    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the builder as a template for constructing
    // many Foos.
    fn finish(&self) -> Foo {
        // Create a Foo from the FooBuilder, applying all settings in FooBuilder to Foo.
    }
}

fn main() {
    let f = FooBuilder::new().named("Bar").with_attribute(...).finish();
}

```


## Motivation

Useful when you would otherwise require many different constructors or where
construction has side effects.


## Advantages

Separates methods for building from other methods.

Prevents proliferation of constructors

Can be used for one-liner initialisation as well as more complex construction.


## Disadvantages

More complex than creating a struct object directly, or a simple constructor
function.


## Discussion

This pattern is seen more frequently in Rust (and for simpler objects) than in
many other languages because Rust lacks overloading. Since you can only have a
single method with a given name, having multiple constructors is less nice in
Rust than in C++, Java, or others.

This pattern is often used where the builder object is useful in its own right,
rather than being just a builder. For example, see `std::process::Command` is a
builder for `Child` (a process). In these cases, the `T` and `TBuilder` pattern
of naming is not used.

The example takes and returns the builder by value. It is often more ergonomic
(and more efficient) to take and return the builder as a mutable reference. The
borrow checker makes this work naturally. This approach has the advantage that
one can write code like

```
let mut fb = FooBuilder::new();
fb.a();
fb.b();
let f = fb.finish();
```

as well as the `FooBuilder::new().a().b().finish()` style.

## See also

[Description in the style guide](http://doc.rust-lang.org/stable/style/ownership/builders.html)

[Constructor pattern](../idioms/ctor.md) for when construction is simpler.

[Builder pattern (wikipedia)](https://en.wikipedia.org/wiki/Builder_pattern)
