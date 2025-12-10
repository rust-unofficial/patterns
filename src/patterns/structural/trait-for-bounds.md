# Use custom traits to avoid complex type bounds

## Description

Trait bounds can become somewhat unwieldy, especially if one of the `Fn` traits[^fn-traits]
is involved and there are specific requirements on the output type. In such
cases the introduction of a new trait may help reduce verbosity, eliminate some
type parameters and thus increase expressiveness. Such a trait can be
accompanied with a generic `impl` for all types satisfying the original bound.

## Example

Let's imagine some sort of monitoring or information gathering system. The
system retrieves values of various types from diverse sources. It may derive
from them some sort of status indicating issues. For example, the total amount
of free memory should be above a certain theshold, and the user with the id `0`
should always be named "root".

For management reasons, we probably want type erasure on the top level. However,
we also need to provide specific (user configurable) assesments for specific
types of data sources (e.g. thresholds and ranges for numerical types). And
since sources for these values are diverse, we may choose to supply data sources
as closures that return a value when called. Because we are probably getting
those values from the operating system, we are likely confronted with operations
that may fail.

We thus may have settled on the following types and traits for handling specific
values:

```rust
use std::fmt::Display;

struct Value<G: FnMut() -> Result<T, Error>, S: Fn(&T) -> Status, T: Display> {
    value: Option<T>,
    getter: G,
    status: S,
}

impl<G: FnMut() -> Result<T, Error>, S: Fn(&T) -> Status, T: Display> Value<G, S, T> {
    pub fn update(&mut self) -> Result<(), Error> {
        (self.getter)().map(|v| self.value = Some(v))
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn status(&self) -> Option<Status> {
        self.value().map(&self.status)
    }
}

// ...

enum Status {
    // ...
}

struct Error {
    // ...
}
```

With these types, we will need to repeat the trait bounds for `G` in at least a
few places. Readability suffers, partially due the the fact that the getter
returns a `Result`. Introducing a bound for "getters" allows a more expressive
bound and eliminate one of the type parameters:

```rust
# use std::fmt::Display;
trait Getter {
    type Output: Display;

    fn get_value(&mut self) -> Result<Self::Output, Error>;
}

impl<F: FnMut() -> Result<T, Error>, T: Display> Getter for F {
    type Output = T;

    fn get_value(&mut self) -> Result<Self::Output, Error> {
        self()
    }
}

struct Value<G: Getter, S: Fn(&G::Output) -> Status> {
    value: Option<G::Output>,
    getter: G,
    status: S,
}

// ...
# enum Status {}
# struct Error;
```

## Advantages

Introducing a new trait can help simplify type bounds, particularly via the
elimination of type parameters. A good name for the new trait will also make the
bound more expressive. The new trait, an abstraction, also offers opportunities
in itself, including:

- additional, specialized types implementing the new trait (e.g. representing an
  idendity of some sort) as well as other useful traits such as `Default` and
- additional methods, as long as they can be implemented for all relevant types.

## Disadvantages

Introducing new items such as the trait means we need to find an appropriate
name and place for it. It also means one more item users of the original
functionality need to investigate[^read-docs]. Depending on presentation, it may
not be obvious right away that a simple closure may be used as a `Getter` in the
example above.

[^fn-traits]: i.e. `Fn`, `FnOnce` and `FnMut`

[^read-docs]: meaning they may need to read more documentation
