# Strategy (aka Policy)

## Description

The [Strategy design pattern](https://en.wikipedia.org/wiki/Strategy_pattern) is a technique that enables separation of concerns in object-oriented software design.
It also allows to decouple software modules through [Dependency Inversion](https://en.wikipedia.org/wiki/Dependency_inversion_principle).
The basic idea behind the Strategy pattern is that, given an algorithm solving a particular problem, we define only the skeleton of the algorithm at an abstract level and we delegate the specific algorithm’s implementation (or some of its parts) to another class or method.

In this way, a client using the algorithm may choose a specific implementation, while the general algorithm workflow remains the same.
In other words, the abstract specification of the class does not depend on the specific implementation of the derived class, but specific implementation must adhere to the abstract specification.
This is why we call it "Dependency Inversion".

## Motivation

Imagine we are working on a project that generates reports every month.
We need the reports to be generated in different formats (strategies), e.g.,
in `JSON` or `Plain Text` formats.
But things vary over time and we don't know what kind of requirement we may get in the future.
For example, we may need to generate our report in a completly new format,
or just modify one of the existing formats.

## Example

In this example our invariants (or abstractions) are `Context`, `Formatter`, and `Report`,
while `Text` and `Json` are our strategy structs.
These strategies have to implement the `Formatter` trait.

```rust
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::{error, result};
type Result = result::Result<String, Box<dyn error::Error>>;
type Data = HashMap<String, u32>;

trait Formatter {
    fn run(&self, data: &Data) -> Result;
}

struct Report;

impl Report {
    fn generate<T: Formatter>(g: T) -> Result {
        // backend operations...
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        // generate report
        g.run(&data)
    }
}

struct Text;
impl Formatter for Text {
    fn run(&self, data: &Data) -> Result {
        let mut s = String::new();

        for (key, val) in data {
            write!(&mut s, "{} {}\n", key, val)?;
        }
        Ok(s)
    }
}

struct Json;
impl Formatter for Json {
    fn run(&self, data: &Data) -> Result {
        let mut s = String::from("[");

        for (key, val) in data {
            if s.len() > 1 {
                write!(&mut s, ",")?;
            }
            write!(&mut s, "{{\"{}\":\"{}\"}}", key, val)?;
        }
        write!(&mut s, "]")?;
        Ok(s)
    }
}

fn main() {
    let s = Report::generate(Text).unwrap();
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    let s = Report::generate(Json).unwrap();
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
```

## Advantages

Separation of concerns. In the previous example, Report does not know anything about
specific implementations of `Json` and `Text`, whereas the output implementations does not care about how data is preprocessed, stored, and fetched. 
The only thing they have to know is context and and a specific trait and method to implement, i.e., `Formatter` and `run`.

## Disadvantages

For each strategy there must me implemented at least one module, so number of modules
increases with number of strategies.
If there are many strategies to choose from then users have to know how strategies differ
from one another.

## Discussion

In the previous example all strategies in a single file.
Ways of providing different strategies includes:

- All in one file (as shown in this example, similar to being separated as modules)
- Separated as modules, E.g. `formatter::json` module, `formatter::text` module
- Use compiler feature flags, E.g. `json` feature, `text` feature
- Separated as crates, E.g. `json` crate, `text` crate

However, we don't need to use traits in order to design this pattern in Rust.

The following toy example demonstrates the idea of the Strategy pattern using Rust
`closures`:

```rust
struct Adder;
impl Adder {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8,
    {
        f(x, y)
    }
}

fn main() {
    let arith_adder = |x, y| x + y;
    let bool_adder = |x, y| {
        if x == 1 || y == 1 {
            1
        } else {
            0
        }
    };
    let custom_adder = |x, y| 2 * x + y;

    assert_eq!(9, Adder::add(4, 5, arith_adder));
    assert_eq!(0, Adder::add(0, 0, bool_adder));
    assert_eq!(5, Adder::add(1, 3, custom_adder));
}

```

In fact, Rust already uses this idea for `Options`'s `map` method

```rust
fn main() {
    let val = Some("Rust");
    let len_strategy = |s: &str| s.len();
    let first_byte_strategy = |s: &str| s.bytes().next().unwrap();

    assert_eq!(4, val.map(len_strategy).unwrap());
    assert_eq!(82, val.map(first_byte_strategy).unwrap());
}
```
