# Lenses and Prisms

This is a pure functional concept that is not frequently used in Rust, because
other idioms can achieve the same result and more advanced forms are not
supported.
Never the less, exploring the concept may be helpful to understand other
patterns in Rust APIs, such as
[visitors](../../patterns/behavioural/visitor.md) and "callbacks".
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
  "accounts"; [
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
fn unique_ids_boxed<I>(iterator: I) -> HashSet<u64> where I: Iterator<Item=Box<dyn CustomerId>> {
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
   traits allow mutation of types, the lense concept is designed to make deep
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

// the type of the lense itself, which is currently just a type alias unlike
// functional languages
/*
type LenseFn<T> = dyn FnOnce(&dyn RecordType) -> Option<T>;
*/
// types that the lense can "view":
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

// this is where a lense can be used in a generic way, since "unique ID"
// need not be the same kind of ID if they are all u64s
fn unique_ids<I>(iterator: I) -> HashSet<u64>
    where I: Iterator<Item=Box<dyn Any>>
{
    // would be: iterator.filter_map(|it| it.view(get_customer_data)).collect()
    iterator.map(get_customer_data).filter_map(|uniq_id_opt| uniq_id_opt).collect()
}
```

While it would be much cleaner if Rust supported more funcitonal traits (a tall
order indeed), hopefully the general idea is clear.

## Modification with Lenses

The functional approach need not be restricted to accessing members.
More powerful lenses can be created which both set and get data in a structure.
This becomes more interesting when examining how these lenses can be composed
together.

to begin, consider a new lens trait, this one to update the balance of a bank
account.
these accounts have different structures for individuals and commercial
business customers.

```rust,ignore
pub struct Eur(u64); // 100 = 1 €

// type castings to integers and display functions omitted

pub trait balancechange {
    fn balance(&self) -> Eur;
    fn deposit(&mut self, Eur);
    fn withdraw(&mut self, Eur);
}
```

This allows for *composition* between two lenses.
It looks very different in haskell, so here is the rust way:

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

The other ways this can be done is with composition functions, such as `map`
and `and_then` on optional types.

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
Consider the `Deserializer' trait:

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

This API is a prism.

To understand it better, consider what "light" it is "refracting".
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

Thinking back to `BalanceChange` earlier, this trait acts as a lens over a data
structure currently being composed.

For example, consider the identity record from earlier but simplified:

```json
{ "name": "Jane Doe",
  "customer_id": 1048576332,
}
```

In order to parse this record, a `Visitor` would be created that expects a
particular sequence of calls:

1. Begin visiting sequence.
1. Visit string "name".
1. Visit unknown string.
1. Combine into sequence key-value pair.
1. Visit string "customer_id".
1. Visit unknown string.
1. Combine into sequence key-value pair.
1. End visiting sequence.

If the wrong call happens, an error is thrown and deserialization fails.

With each new type, a new implementor of the `Visitor` trait is needed.
Serde does this clearly and secretly using a derive macro:

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
The way it does so is it creates and operates a `Visitor`.

A simple example follows of a `Deserializer` that knows how to parse a single
32-bit binary number written out with ASCII 0s and 1s.
For a better example, see the Serde documentation.

Note how this is composable in many other formats without predicting anything
else about their structure.

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

The only reason this works is due to a subtlety of the `Deserializer` trait's
definition: the output type **is specified by the implementor of `Visitor` it
is passed**.
This was *not* true in the account example earlier.

This means that a single call -- which drives a `Deserializer` -- will produce
different output types depending on what visitor it was given, even though the
`Value` type that results is not bound to the `Deserializer` at all!

One might say `Deserializer` is "directly generic" over `Visitor`, and
"indirectly generic` over `Value`.
That means it is a "sum type" -- where a type is "applied" to it from another
type -- and that makes it a prism.

Any non-functional language can only achieve this with hacks: code generation,
run-time polymorphism, duck typing, or constricting the interface of the
output.

But with Rust's generic-inspired type system, it can be just as flexible as
functional languages, and solve this problem while remaining statically typed.

## See Also

- [lens-rs crate](https://crates.io/crates/lens-rs) for a pre-built lenses
  implementation, with a cleaner interface than these examples
- [luminance](https://github.com/phaazon/luminance-rs) is a crate that uses
  lense API design, including proceducal macros to crate full equivalence to
  the missing generic traits mentioned earlier
- [An Article about Lenses in
  Scala](https://medium.com/zyseme-technology/functional-references-lens-and-other-optics-in-scala-e5f7e2fdafe)
  that is very readable even without Scala expertise.
- [Paper: Profunctor Optics: Modular Data
  Accessors](https://arxiv.org/ftp/arxiv/papers/1703/1703.10857.pdf)

[^1] https://www.schoolofhaskell.com/school/to-infinity-and-beyond/pick-of-the-week/a-little-lens-starter-tutorial
