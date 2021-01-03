# On-Stack Dynamic Dispatch

## Description

We can dynamically dispatch over multiple values, however, to do so, we need
to declare multiple variables to bind differently-typed objects. To extend the
lifetime as necessary, we can use deferred conditional initialization, as seen
below:

## Example

```rust
use std::io;
use std::fs;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let arg = "-";

// These must live longer than `readable`, and thus are declared first:
let (mut stdin_read, mut file_read);

// We need to ascribe the type to get dynamic dispatch.
let readable: &mut dyn io::Read = if arg == "-" {
    stdin_read = io::stdin();
    &mut stdin_read
} else {
    file_read = fs::File::open(arg)?;
    &mut file_read
};

// Read from `readable` here.

# Ok(())
# }
```

## Motivation

Rust monomorphises code by default. This means a copy of the code will be
generated for each type it is used with and optimized independently. While this
allows for very fast code on the hot path, it also bloats the code in places
where performance is not of the essence, thus costing compile time and cache
usage.

Luckily, Rust allows us to use dynamic dispatch, but we have to explicitly ask
for it.

## Advantages

We do not need to allocate anything on the heap. Neither do we need to
initialize something we won't use later, nor do we need to monomorphize the
whole code that follows to work with both `File` or `Stdin`.

## Disadvantages

The code needs more moving parts than the `Box`-based version:

```rust,ignore
// We still need to ascribe the type for dynamic dispatch.
let readable: Box<dyn io::Read> = if arg == "-" {
    Box::new(io::stdin())
} else {
    Box::new(fs::File::open(arg)?)
};
// Read from `readable` here.
```

## Discussion

Rust newcomers will usually learn that Rust requires all variables to be
initialized *before use*, so it's easy to overlook the fact that *unused*
variables may well be uninitialized. Rust works quite hard to ensure that this
works out fine and only the initialized values are dropped at the end of their
scope.

The example meets all the constraints Rust places on us:

* All variables are initialized before using (in this case borrowing) them
* Each variable only holds values of a single type. In our example, `stdin` is
of type `Stdin`, `file` is of type `File` and `readable` is of type `&mut dyn
Read`
* Each borrowed value outlives all the references borrowed from it

## See also

* [Finalisation in destructors](dtor-finally.md) and 
[RAII guards](../patterns/RAII.md) can benefit from tight control over lifetimes.
* For conditionally filled `Option<&T>`s of (mutable) references, one can
initialize an `Option<T>` directly and use its [`.as_ref()`] method to get an
optional reference.

[`.as_ref()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref
