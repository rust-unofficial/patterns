# Strategy (aka Policy)

## Description

The [Strategy design pattern](https://en.wikipedia.org/wiki/Strategy_pattern) is a technique that enables separation of concerns in object-oriented software design.
It also allows to achieve decoupling software modules known as [Dependency Inversion Principle](https://en.wikipedia.org/wiki/Dependency_inversion_principle).
The basic idea behind Strategy is given an algorithm, solving a particular problem, we define only skeleton of the algorithm at abstract level, but delegate specific algorithm’s implementation or some of its parts to another class or method.
A client using algorithm may choose a specific implementation, while the general algorithm workflow remains the same.
In other words, the abstract specification of the class does not depend on the specific implementation of the derived class, but specific implementation must adhere to the abstract specification.
This is why we call it Dependency Inversion.

## Motivation

Imagine we are working on a project that generates reports every month. 
We need the reports to be generated in different formats, e.g., in `JSON` or `Plain Text` formats. 
But thing very and we don't know what kind of requiremnt we may get in the future. 
For example, we may need to generate our report in a completly new format, 
or just modify the existing format. 
So, in order to make chages to our existing code base as much as possible 
while keeping our solution adoptable to such changes, 
we might exploit Strategy design pattern. 
Rust has powerful trait system which allows to implement this pattern. 

## Example

In this example our invariants (or abstractions) are `Context`, `Formatter`, and `Report`. 
`Text` and `Json` are our strategy structs. 
These strategies have to implement `Formatter` trait.

```rust

use std::fmt::Write as FmtWrite;
use std::{error, result};
type Result = result::Result<String, Box<dyn error::Error>>;

struct Context {
    pub keys: Vec<String>,
    pub values: Vec<i32>,
}

trait Formatter {
    fn run(&self, context: &Context) -> Result;
}

struct Report;

impl Report {
    fn generate<T: Formatter>(g: T) -> Result {
        //perform here backend operations which should not bother caller...
        //fetch data from database
        let keys = vec!["one".to_string(), "two".to_string()];
        let values = vec![1, 2];
        // generate report
        g.run(&Context { keys, values })
    }
}

struct Text;
impl Formatter for Text {
    fn run(&self, context: &Context) -> Result {
        let mut s = String::new();

        for (key, val) in context.keys.iter().zip(context.values.iter()) {
            write!(&mut s, "{} {}\n", key, val)?;
        }
        Ok(s)
    }
}

struct Json;
impl Formatter for Json {
    fn run(&self, context: &Context) -> Result {
        let mut s = String::from("[");

        for (key, val) in context.keys.iter().zip(context.values.iter()) {
            if s.len() > 1 {
                write!(&mut s, ",")?;
            }
            write!(&mut s, "{{ \"{}\":\"{}\"}}", key, val)?;
        }
        write!(&mut s, "]")?;
        Ok(s)
    }
}

fn main() {
    assert_eq!(
        String::from("one 1\ntwo 2\n"),
        Report::generate(Text).unwrap()
    );

    assert_eq!(
        String::from(r#"[{ "one":"1"},{ "two":"2"}]"#),
        Report::generate(Json).unwrap()
    );
}

```


## Advantages

Separation of concerns. In the previous example, Report does not know anything about specific implementations of `Json` and `Text`, whereas the output implementations does not care about how data is preprocessed, stored, and fetched. 
The only thing they have to know is context and and a specific trait and method to implement, i.e., `Formatter` and `run`. 

## Disadvantages

For each strategy there must me implemented at least one module, so number of modules increases with number of strategies.
If there are many strategies to choose from then users have to know how strategies differ from one another.

## Discussion

In the previous example all strategies in a single file.
Typically in Rust, each strategy should be implemented in a separate module file.
However, we don't need to use traits in order to design this pattern in Rust.

The following toy example demonstrates the idea of the Strategy pattern using Rust `closures`:

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
    let bool_adder = |x, y| if x == 1 || y == 1 { 1 } else { 0 };
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
