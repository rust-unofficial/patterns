# Lenses and Prisms

This is a pure functional concept that is not frequently used in Rust.
Never the less, exploring the concept may be helpful to understand other
patterns in Rust APIs, such as [visitors](../patterns/behavioural/visitor.md).
They also have niche use cases.

## Lenses: Uniform Access Across Types

A lens is a concept from functional languages that allows accessing parts of a
data type in an abstract, unified way.[^1]
In basic concept, it is similar to the way Rust traits work with type erasure,
but it has a bit more power and flexibility.

For a basic example, suppose a bank contains several JSON formats for customer
data.
This is because they come from different databases or legacy systems.
One database contains the data needed to perform credit checks:

```json
{ "name": "Jane Doe",
  "dob": "2002-02-24",
  [...]
  "customer_id": 1048576332,
}
```

Another one contains the account information:

```json
{ "customer_id": 1048576332,
  "accounts": [
      { "account_id": 2121,
        "account_type: "savings",
        "joint_customer_ids": [],
        [...]
      },
      { "account_id": 2122,
        "account_type: "checking",
        "joint_customer_ids": [1048576333],
        [...]
      },
  ]
} 
```

Notice that both types have a customer ID number which corresponds to a person.
How would a single function handle both records of different types?

In Rust, a `struct` could represent each of these types, and a trait would have
a `get_customer_id` function they would implement:

```rust
use std::collections::HashSet;

pub struct Account {
    account_id: u32,
    account_type: String,
    // other fields omitted
}

pub trait CustomerId {
    fn get_customer_id(&self) -> u64;
}

pub struct CreditRecord {
    customer_id: u64,
    name: String,
    dob: String,
    // other fields omitted
}

impl CustomerId for CreditRecord {
    fn get_customer_id(&self) -> u64 {
        self.customer_id
    }
}

pub struct AccountRecord {
    customer_id: u64,
    accounts: Vec<Account>,
}

impl CustomerId for AccountRecord {
    fn get_customer_id(&self) -> u64 {
        self.customer_id
    }
}

// static polymorphism: only one type, but each function call can choose it
fn unique_ids_set<R: CustomerId>(records: &[R]) -> HashSet<u64> {
    records.iter().map(|r| r.get_customer_id()).collect()
}

// dynamic dispatch: iterates over any type with a customer ID, collecting all
// values together
fn unique_ids_iter<I>(iterator: I) -> HashSet<u64>
    where I: Iterator<Item=Box<dyn CustomerId>>
{
    iterator.map(|r| r.as_ref().get_customer_id()).collect()
}
```

Lenses, however, allow the code supporting customer ID to be moved from the
*type* to the *accessor function*.
Rather than implementing a trait on each type, all matching structures can
simply be accessed the same way.

While the Rust language itself does not support this (type erasure is the
preferred solution to this problem), the [lens-rs
crate](https://github.com/TOETOE55/lens-rs/blob/master/guide.md) allows code
that feels like this to be written with macros:

```rust,ignore
use std::collections::HashSet;

use lens_rs::{optics, Lens, LensRef, Optics};

#[derive(Clone, Debug, Lens /* derive to allow lenses to work */)]
pub struct CreditRecord {
    #[optic(ref)] // macro attribute to allow viewing this field
    customer_id: u64,
    name: String,
    dob: String,
    // other fields omitted
}

#[derive(Clone, Debug)]
pub struct Account {
    account_id: u32,
    account_type: String,
    // other fields omitted
}

#[derive(Clone, Debug, Lens)]
pub struct AccountRecord {
    #[optic(ref)]
    customer_id: u64,
    accounts: Vec<Account>,
}

fn unique_ids_lens<T>(iter: impl Iterator<Item = T>) -> HashSet<u64>
where
    T: LensRef<Optics![customer_id], u64>, // any type with this field
{
    iter.map(|r| *r.view_ref(optics!(customer_id))).collect()
}
```

The version of `unique_ids` shown here allows any type to be in the iterator,
so long as it has an attribute called `customer_id` which can be accessed by
the function.
This is how most functional languages operate on lenses.

Rather than macros, they achieve this with a technique known as "currying".
That is, they "partially construct" the function, leaving the type of the
final parameter (the value being operated on) unfilled until the function is
called.
Thus it can be called with different types dynamically even from one place in
the code.
That is what the `optics!` and `view_ref` in the code above simulates.

The functional approach need not be restricted to accessing members.
More powerful lenses can be created which both set and get data in a
structure.
But the concept really becomes interesting when used as a building block for
composition.
That is where the concept appears more clearly in Rust.

## Prisms: A Higher-Order form of "Optics"

A simple function such as `unique_ids` above operates on a single lens.
A *prism* is a function that operates on a *family* of lenses.
It is one conceptual level higher, using lenses as a building block, and
continuing the metaphor, is part of a family of "optics".
It is the main one that is useful in understanding Rust APIs, so will be the
focus here.

The same way that traits allow "lens-like" design with static polymorphism and
dynamic dispatch, prism-like designs appear in Rust APIs which split problems
into multiple associated types to be composed.
A good example of this is the traits in the parsing crate Serde.

Trying to understand the way Serde works by only reading the API is a
challenge, especially the first time.
Consider the `Deserializer` trait, implemented by some type in any library
which parses a new format:

```rust,ignore
pub trait Deserializer<'de>: Sized {
    type Error: Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    // remainder ommitted
}
```

For a trait that's just supposed to parse data from a format and return a
value, this looks odd.
Why are all the return types type erased?

To understand, keep the lens concept in mind and look at the definition of
the `Visitor` type that is passed in generically:

```rust,ignore
pub trait Visitor<'de>: Sized {
    type Value;

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error;

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error;

    // remainder omitted
}
```

The job of the `Visitor` type is to construct values in the Serde data model,
which are represented by its associated `Value` type.

These values represent parts of the Rust value being deserialized.
If this fails, it returns an `Error` type -- an error type determined by the
`Deserializer` when its methods were called.

This highlights that `Deserializer` is similar to `CustomerId` from earlier,
allowing any format parser which implements it to create `Value`s based on what
it parsed.
The `Value` trait is acting like a lens in functional languages.

But unlike the `CustomerId` trait, the return types of `Visitor` methods are
*generic*, and the concrete `Value` type is *determined by the Visitor itself*.

Instead of acting as one lens, it effectively a family of
lenses, one for each concrete type of `Visitor`.

The `Deserializer` API is based on having a generic set of "lenses" work across
a set of other generic types for "observation".
It is a *prism*.

For example, consider the identity record from earlier but simplified:

```json
{ "name": "Jane Doe",
  "customer_id": 1048576332,
}
```

How would the *Serde* library deserialize this JSON into `struct CreditRecord`?

1. The user would call a library function to deserialize the data. This would
   create a `Deserializer` based on the JSON format.
1. Based on the fields in the struct, a `Visitor` would be created (more on
   that in a moment) which knows how to create each type in a generic data
   model that was needed to represent it: `u64` and `String`.
1. The deserializer would make calls to the `Visitor` as it parsed items.
1. The `Visitor` would indicate if the items found were expected, and if not,
   raise an error to indicate deserialization has failed.

For our very simple structure above, the expected pattern would be:

1. Visit a map (Serde's equvialent to `HashMap` or JSON's dictionary).
1. Visit a string key called "name".
1. Visit a string value, which will go into the `name` field.
1. Visit a string key called "customer_id".
1. Visit a string value, which will go into the `customer_id` field.
1. Visit the end of the map.

But what determines which "observation" pattern is expected?
A functional language would be able to use currying to create reflection of
each type based on the type itself.
Rust does not support that, so every single type would need to have its own
code written based on its fields and their properties.

Serde solves this usability problem with a derive macro:

```rust,ignore
use serde::Deserialize;

#[derive(Deserialize)]
struct IdRecord {
    name: String,
    customer_id: String,
}
```

That macro simply generates an impl block causing the struct to implement a
trait called `Deserialize`.
It is defined this way:

```rust,ignore
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

This is the function that determines how to create the struct itself.
Code is generated based on the struct's fields.
When the parsing library is called -- in our example, a JSON parsing library --
it creates a `Deserializer` and calls `Type::deserialize` with it as a
parameter.

The `deserialize` code will then create a `Visitor` which will have its calls
"refracted" by the `Deserializer`.
If everything goes well, eventually that `Visitor` will construct a value
corresponding to the type being parsed and return it.

For a complete example, see the [Serde
documentation](https://serde.rs/deserialize-struct.html).

To wrap up, this is the power of Serde:

1. The structure being parsed is represented by an impl block for `Deserialize`
1. The input data format (e.g. JSON) is represented by a `Deserializer` called
   by `Deserialize`
1. The `Deserializer` acts like a prism which "refracts" lens-like `Visitor`
   calls which actually build the data value

The result is that types to be deserialized only implement the "top layer" of
the API, and file formats only need to implement the "bottom layer".
Each piece can then "just work" with the rest of the ecosystem, since generic
types will bridge them.

To emphasize, the only reason this model works on any format and any type is
because the `Deserializer` trait's output type **is specified by the
implementor of `Visitor` it is passed**, rather than being tied to one specific
type.
This was not true in the account example earlier.

Rust's generic-inspired type system can bring it close to these concepts and
use their power, as shown in this API design.
But it may also need procedural macros to create bridges for its generics.

## See Also

- [lens-rs crate](https://crates.io/crates/lens-rs) for a pre-built lenses
  implementation, with a cleaner interface than these examples
- [serde](https://serde.rs) itself, which makes these concepts intuitive for
  end users (i.e. defining the structs) without needing to undestand the
  details
- [luminance](https://github.com/phaazon/luminance-rs) is a crate for drawing
  computer graphics that uses lens API design, including proceducal macros to
  create full prisms for buffers of different pixel types that remain generic
- [An Article about Lenses in
  Scala](https://web.archive.org/web/20221128185849/https://medium.com/zyseme-technology/functional-references-lens-and-other-optics-in-scala-e5f7e2fdafe)
  that is very readable even without Scala expertise.
- [Paper: Profunctor Optics: Modular Data
  Accessors](https://web.archive.org/web/20220701102832/https://arxiv.org/ftp/arxiv/papers/1703/1703.10857.pdf)

[^1]: [School of Haskell: A Little Lens Starter Tutorial](https://web.archive.org/web/20221128190041/https://www.schoolofhaskell.com/school/to-infinity-and-beyond/pick-of-the-week/a-little-lens-starter-tutorial)
