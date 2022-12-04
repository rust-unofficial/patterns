# Lenses and Prisms

This is a pure functional concept that is not frequently used in Rust, because
other idioms can achieve the same result and more advanced forms are not
supported.
Never the less, exploring the concept may be helpful to understand other
patterns in Rust APIs, such as
[visitors](../patterns/behavioural/visitor.md) and "callbacks".
They also have niche use cases.

In addition, async Rust may be considered somewhat "lens like" in many of the
APIs that exist today, e.g. `Future`.

## Basic Lenses: Like Rust Traits

A basic lens allows composability similar to Rust traits instead of concrete types.

For example, suppose a bank contains several JSON formats for customer data.
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
{ "primary_customer_id": 1048576332,
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

It would be possible to create a `struct` for each of these types and parse the
JSON into it.
However, each one would need to implement their own version of, for example,
`get_customer_id`.
But what if one piece of code wanted to retrieve all customer IDs regardless of
data type?

The solution in Rust is to make a trait which represents the operation.
If it is implemented for both `struct`s, then type erasure can make the once
piece of code work:

```rust
use std::collections::HashSet;

pub trait CustomerId {
    fn get_customer_id(&self) -> u64;
}

pub struct CreditRecord {
    customer_id: u64,
    name: String,
    dob: String,
    // other fields omitted
}

pub struct Account {
    account_id: u32,
    account_type: String,
    // other fields omitted
}

pub struct AccountRecord {
    primary_customer_id: u64,
    accounts: Vec<Account>,
}

impl CustomerId for CreditRecord {
    fn get_customer_id(&self) -> u64 {
        self.customer_id
    }
}

impl CustomerId for AccountRecord {
    fn get_customer_id(&self) -> u64 {
        self.primary_customer_id
    }
}

// static polymorphism: allows one singular type at a time, but a function call
// with any type
fn unique_ids_set<R: CustomerId>(records: &[R]) -> HashSet<u64> {
    records.iter().map(|r| r.get_customer_id()).collect()
}

// dynamic polymorphism: iterates over any type, allowing collecting different
// records together
fn unique_ids_boxed<I>(iterator: I) -> HashSet<u64>
    where I: Iterator<Item=Box<dyn CustomerId>>
{
    iterator.map(|r| r.as_ref().get_customer_id()).collect()
}
```

The versions of `unique_ids` shown here implement the *concept* of a lens.

Lenses are a way to allow accessing parts of a data type in an abstract,
unified way.[^1]
Lenses define a generic operation -- in our case, getting a customer ID from a
record -- that can apply to multiple types.
This is so even though each type has a different way to "observe" the desired
data.

At first glance, it may seem more like Rust is using "duck typing", the way
that dynamic languages do.
However, there are significant differences:

1. Static type checking is maintained. Only types which "opt-in" to having a
   customer ID (with an `impl CustomerId` block) can be placed into the
   iterator or collection. Duck typed languages require this to be a runtime
   check, which can then go wrong at runtime.
1. The `impl CustomerId` block lives with the type. This means the type can
   live in a different crate than definition of `CustomerId` itself,
   maintaining extensibility.
1. The definition of `CustomerId` is consistent throughout the program. Unlike
   some approaches functional languages take (discussed below), it is
   impossible to accidentally change the type in such a way the trait becomes
   "invalid" at runtime.
1. In many functional languages, data structures are immutable. While Rust's
   traits allow mutation of types, the lens concept is designed to make deep
   copying types with small changes much easier. (Deep copies which are often
   optimized differently by the runtime. but that's another story.)

Most functional languages take a different approach to this concept, however.
They prefer instead to use other features Rust does not have -- a form of
partial function construction known as "currying" -- to do it with functions
instead.
They "partially construct" the function, leaving the type of the object being
operated on until the function is constructed, which Rust cannot do.

This is the nearest thing to a Rust equivalent to the "pure functional way":

```rust
use std::any::Any;
use std::collections::HashSet;

pub trait CustomerId {
    fn get_customer_id(&self) -> u64;
}

pub struct CreditRecord {
    customer_id: u64,
    name: String,
    dob: String,
    // other fields omitted
}

pub struct Account {
    account_id: u32,
    account_type: String,
    // other fields omitted
}

pub struct AccountRecord {
    primary_customer_id: u64,
    accounts: Vec<Account>,
}

// the type of the lens itself, which is currently just a type alias unlike
// functional languages
/*
type LenseFn<T> = dyn FnOnce(&dyn RecordType) -> Option<T>;
*/
// types that the lens can "view":
/*
trait RecordType {
    fn view<T, L>(&self, lens: L) -> T
        where L: FnOnce(&dyn RecordType) -> Option<T>
    {
        (lens)(self)
    }
}
*/
// all this is commented out because Rust does not support generics in traits
// instead we have to implement that same function on the different types
// but the lenses themselves should still be possible
trait  RecordType {}
impl RecordType for CreditRecord {}
impl RecordType for AccountRecord {}
impl CreditRecord {
    fn view<T, L>(&self, lens: L) -> Option<T>
        where L: FnOnce(&CreditRecord) -> Option<T>
    {
        (lens)(self)
    }
}
impl AccountRecord {
    fn view<T, L>(&self, lens: L) -> Option<T>
        where L: FnOnce(&AccountRecord) -> Option<T>
    {
        (lens)(self)
    }
}

// an example lense
fn get_customer_data(
    item: Box<dyn Any> /* would be &dyn RecordType */
) -> Option<u64> {
    // more hacking around no generics in types, focus on the fn signature
    let item = match item.downcast::<CreditRecord>() {
        Ok(credit) => return Some(credit.customer_id),
        Err(bx) => bx
    };
    let item = match item.downcast::<AccountRecord>() {
        Ok(acct) => return Some(acct.primary_customer_id),
        Err(bx) => bx
    };
    None
}

// this is where a lens can be used in a generic way, since "unique ID"
// need not be the same kind of ID if they are all u64s
fn unique_ids<I>(iterator: I) -> HashSet<u64>
    where I: Iterator<Item=Box<dyn Any>>
{
    // would be: iterator.filter_map(|it| it.view(get_customer_data)).collect()
    iterator.map(get_customer_data).filter_map(|uniq_id_opt| uniq_id_opt).collect()
}
```

While it would be much cleaner if Rust supported more functional traits (a tall
order indeed), hopefully the general idea is clear.

## Modification with Lenses

The functional approach need not be restricted to accessing members.
More powerful lenses can be created which both set and get data in a structure.
This becomes more interesting when examining how these lenses can be composed
together.

To begin, consider a new lens trait, this one to update the balance of a bank
account.
These accounts have different structures for individuals and commercial
business customers.

```rust,ignore
pub struct Eur(u64); // 100 = 1 €

// type casting to integers and display functions omitted

pub trait BalanceChange {
    fn balance(&self) -> Eur;
    fn deposit(&mut self, Eur);
    fn withdraw(&mut self, Eur);
}
```

This trait allows *composition* between two lenses.
It looks very different in Haskell, so here is the Rust way:

```rust,ignore
use chrono::prelude::*;
use rand::{RngCore, thread_rng};
use sha2::{Sha256, Digest};

fn make_id_deposit<A: BalanceChange + CustomerId>(account: &mut A) {
    // this computes a hash based on the current balance and the account ID,
    // resulting in a pair of transactions that the customer must use to verify
    // their identity when resetting their pin via telephone

    let balance = account.balance();
    let id = account.get_customer_id();
    let date = Utc::now().timestamp() as usize;
    let secure: u64 = thread_rng().next_u64();

    let mut hasher = Sha256::new();
    hasher.update(
        format!("{:?},{},{},{}", balance, id, date, secure).as_bytes());
    let cents_hash = &hasher.finalize()[0..1];

    // example: 1234 results in deposits of 0.12 and 0.34
    let cents = i16::from_be_bytes([cents_hash[0], cents_hash[1]]) as u64 % 1000;
    account.deposit(Eur(cents / 100));
    account.deposit(Eur(cents % 100));
}
```

Here the composition is done through the generic `A`, which is defined as types
with both traits.
In order to literally compose lens functions, Rust uses `map` and `and_then`
on `Option<T>` and iterator, which would allow each lens to be called
sequentially on items.

## Prisms: Lenses, but Sum Types

A prism is like a lens, but one conceptual level higher.
It is a way to make a high-level, trait-like interface apply across different
lenses.

While pure functional languages have many more uses, a primary use for this in
Rust is to split problems of recursion and types into pieces that can be
composed.
A good example of this is the traits in Serde.

Trying to understand the way Serde works by reading just the API is quite a
challenge.
Consider the `Deserializer` trait:

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

Wow, that's a lot of generics on functions, in addition to a type on the trait
itself!
Why are they all there?

They are there because this API is a prism: an attempt to make a generic set of
"lenses" work across a generic set of types for "observation."

To understand it better, consider what each "lense" looks like.
Here is the definition of the `Visitor` type:

```rust,ignore
pub trait Visitor<'de>: Sized {
    type Value;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> Result;

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error;

    // remainder omitted
}
```

This is similar to `BalanceChange` earlier. It is acting as a lens over a
generic and unknown type of data.
Its operation is to support composing a set of uniform data structures from
that data represented by Serde's `Value` type.

But unlike `BalanceChange` it's a *trait*.
It's a *family* of lenses.

For example, consider the identity record from earlier but simplified:

```json
{ "name": "Jane Doe",
  "customer_id": 1048576332,
}
```

How would the Serde library transform this JSON into `struct CreditRecord`?

It would create a `Visitor` "lens", which would then try to "observe" expected
data through a series of calls:

1. Visit the start of a map (the `{}` in Serde terminology).
1. Visit a key string called "name".
1. Visit an unknown string which is the value of the `name` field.
1. Visit a key string called "customer_id".
1. Visit an unknown string which is the value of the `customer_id` field.
1. Expect the end of the map.
1. Visit complete.

If the wrong "observation" happens, an error is thrown and deserialization
fails.

With each new type, a new implementation of the `Visitor` trait is needed,
a type to explain what to observe in what order to fill out the struct.
While Rust does not support generic traits as noted earlier, Serde solves this
usability problem with a derive procedural macro:

```rust,ignore
use serde::Deserialize;

#[derive(Deserialize)]
struct IdRecord {
    name: String,
    customer_id: String,
}
```

In C++, the equivalent would be to generate a very complex code template, with
a lot of compile time logic -- if it's even possible.
But in Rust, that macro simply creates the ability to pass it to a
`Deserializer`.

What is a Deserializer?
It is the logic that contains the instructions to parse a data format, such as
JSON or CSV.
It is "observed" by the lense, and "refracts" the generic calls into calls that
are specific to the data format.

A simple example follows of a `Deserializer` that knows how to parse a single
32-bit binary number written out with ASCII 0s and 1s.
For a better example, see the Serde documentation.

```rust,ignore
struct BinaryDes<'de> {
    input: &'de str,
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut BinaryDes<'de> {
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value> {
        let mut n = 0;
        for ch in self.input.chars() {
            match ch {
             '0' => n <<= 1,
             '1' => n = (n << 1 | 1),
             _ => return Err(Error::ExpectedU32),
            }
        }
        visitor.visit_u32(n)
    }
}

```

Note how this is composable in many formats without predicting anything else
about their structure.
That is the power of prisms.

Key to this example is to a subtlety of the `Deserializer` trait's definition:
the output type **is specified by the implementor of `Visitor` it is passed**.
This was *not* true in the account example earlier, and part of the *family*
definition that makes it a prism.

This means that a single call -- which drives a `Deserializer` -- will produce
different output types depending on what visitor it was given, even though the
`Value` type that results is not bound to the `Deserializer` at all!

One might say `Deserializer` is "directly generic" over `Visitor`, and
"indirectly generic" over `Value`.
That means it is a "sum type" -- where a type is "applied" to it from another
type -- and that makes it a prism.

Most non-functional languages achieve this with hacks: code generation,
run-time polymorphism, duck typing, or constricting the interface of the
output.

But with Rust's generic-inspired type system, it can get a lot closer to
functional languages, and maintain static typing -- even though it may also
need procedural macros to create prisms.

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
  Scala](https://medium.com/zyseme-technology/functional-references-lens-and-other-optics-in-scala-e5f7e2fdafe)
  that is very readable even without Scala expertise.
- [Paper: Profunctor Optics: Modular Data
  Accessors](https://arxiv.org/ftp/arxiv/papers/1703/1703.10857.pdf)

[^1] [https://www.schoolofhaskell.com/school/to-infinity-and-beyond/pick-of-the-week/a-little-lens-starter-tutorial](https://www.schoolofhaskell.com/school/to-infinity-and-beyond/pick-of-the-week/a-little-lens-starter-tutorial)
