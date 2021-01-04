# Strategy (aka Policy)

## Description

The [Strategy design pattern](https://en.wikipedia.org/wiki/Strategy_pattern) is a technique that enables separation of concerns in object-oriented software design.
It also allows to achieve decoupling software modules known as [Dependency Inversion Principle](https://en.wikipedia.org/wiki/Dependency_inversion_principle).
The basic idea behind Strategy is given an algorithm, solving a particular problem, we define only skeleton of the algorithm at abstract level, but delegate specific algorithm’s implementation or some of its parts to another class or method.
A client using algorithm may choose a specific implementation, while the general algorithm workflow remains the same.
In other words, the abstract specification of the class does not depend on the specific implementation of the derived class, but specific implementation must adhere to the abstract specification.
This is why we call it Dependency Inversion.

## Motivation

Suppose we are working on a project whose purpose is periodically read big data, [wrangle](https://en.wikipedia.org/wiki/Data_wrangling) it, and finally store it in a database as key-value pairs.
After the data are stored in database, we need to provide conumers with APIs to query data.
However, different consumers may want data in different formats (json, txt, xml, etc), even in new one we have never seen so far. But we don't know in advance in what format consumers would query data.
So, every time a request for a new format implementation comes, we have to rebuild our software.
We'd also like to change our code as little as possible.
Ideally, the wrangling part of our softaware shouldn't change because it has nothing to with data serialization.
This is a one scenario where we could apply the Strategy pattern to design our software.
We could aslo publish the wrangling part as a library leaving the serialization implementation to library users. The following example explains the idea.

## Example

```rust
// This trait must be implemented by different formats
pub trait Formatter {
  fn run(&self, report: &Report);
}

// STRATEGIES:

// Implementation for Json format
mod vendor1 {
  use super::*;

  pub struct Json;

  impl Formatter for Json {
    fn run(&self, report: &Report) {
      print!("[");
      for (key, val) in report.keys.iter().zip(report.values.iter()) {
        print!("{{ \"{}\":\"{}\"}},", key, val);
      }
      println!("\u{8}]");
    }
  }
}

// Implementation for Plain Text format
mod vendor2 {
  use super::*;

  pub struct Text;

  impl Formatter for Text {
    fn run(&self, report: &Report) {
      for (key, val) in report.keys.iter().zip(report.values.iter()) {
        println!("{} {}", key, val);
      }
    }
  }
}


// Report does not implement Generator trait
// Instead, it provides high level abstraction
// - vector of Strings: `keys`
// - vector of integers: `values`
// - method: `generate`
// 
// In other words, Report does not depend on format implementation,
// but format implementations depend on Report (abtraction). This is called 
// Dependency Inversion Principle. 

pub struct Report {
  pub keys:   Vec<String>,
  pub values: Vec<i32>,
  // User must provide an object which implements Generator trait
  formatter: Box<dyn Formatter>
}

impl Report {
  pub fn format(&self) {
    self.formatter.run(&self);
  }

  pub fn new(formatter: Box<dyn Formatter>) -> Self {
    let (keys, values) = Self::data_from_db();
    Report{ keys, values, formatter }
  }

  fn data_from_db() -> (Vec<String>, Vec<i32>) {
    let keys = vec!["one".to_string(), "two".to_string()];
    let values =  vec![1, 2];
    (keys, values)
  }
}

fn main() {
  let json_report = Report::new(Box::new(vendor1::Json));
  let text_report = Report::new(Box::new(vendor2::Text));

  println!("JSON format:");
  json_report.format();
  
  println!("\n");

  println!("Plain text format:");
  text_report.format();
}

```

## Advantages

Separation of concerns. In the previous example, Report does not know anything about specific implementations of `Vendor1::Json` and `Vendor2::Text`, whereas the output implementations does not care about how data is preprocessed, stored, and fetched. The only thing they have to know is context and and a specific trait and method to implement, i.e., `Generator` and `run`. 

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
impl Adder{
  pub fn add<F>(x: u8, y: u8, f: F) -> u8
      where F: Fn(u8, u8) -> u8 {
      f(x,y)
  }
}

fn main() {
  let arith_adder = |x, y| x + y;
  let bool_adder = |x, y| if x==1 || y==1 {1} else {0};
  let custom_adder = |x, y| 2*x + y;

  println!("{:?}", Adder::add(4, 5, arith_adder));
  println!("{:?}", Adder::add(0, 0, bool_adder));
  println!("{:?}", Adder::add(0, 3, custom_adder));
}
```

In fact, Rust already uses this idea for `Options`'s `map` method

```rust
fn main() {
  let val = Some("Rust");
  let len_strategy = |s: &str| s.len();
  let first_byte_strategy = |s: &str| s.bytes().next().unwrap();

  println!("len: {}", val.map(len_strategy).unwrap());
  println!("first bite: {}", val.map(first_byte_strategy).unwrap());
}
```

### Getting rid of trait objects

In the first example we defined the `Report` as

```rust,ignore
pub struct Report {
  pub keys:   Vec<String>,
  pub values: Vec<i32>,
  // User must provide an object which implements Generator trait
  formatter: Box<dyn Formatter>
}
```

which means we use trait objects and hence dynamic dispatch. However, we can implement the text formatter example using static disptach which is more preferable.

```rust

struct Context {
  pub keys:   Vec<String>,
  pub values: Vec<i32>
}

trait Formatter {
  fn run(&self, context: &Context);
}

struct Report;

impl Report {
  fn generate<T: Formatter>(g: T) {
    //perform backend operations which should not bother caller...
    //fetch data from database
    let keys = vec!["one".to_string(), "two".to_string()];
    let values =  vec![1, 2];
    // generate 
    g.run(&Context{ keys, values });
  }
}


struct Text;
impl Formatter for Text {
  fn run(&self, context: &Context) {
    for (key, val) in context.keys.iter().zip(context.values.iter()) {
      println!("{} {}", key, val);
    }
  }
}

struct Json;
impl Formatter for Json {
  fn run(&self, context: &Context) {
    print!("[");
    for (key, val) in context.keys.iter().zip(context.values.iter()) {
      print!("{{ \"{}\":\"{}\"}},", key, val);
    }
    println!("\u{8}]");
  }
}

fn main() {
  println!("JSON format:");
  Report::generate(Text);
  
  println!("\n");

  println!("Plain text format:");
  Report::generate(Json);
}
```
