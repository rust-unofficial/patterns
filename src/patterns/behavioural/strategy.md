# Strategy (aka Policy)

## Description

The [Strategy design pattern](https://en.wikipedia.org/wiki/Strategy_pattern) is
a technique that enables separation of concerns. It also allows to decouple
software modules through
[Dependency Inversion](https://en.wikipedia.org/wiki/Dependency_inversion_principle).

The basic idea behind the Strategy pattern is that, given an algorithm solving a
particular problem, we define only the skeleton of the algorithm at an abstract
level, and we separate the specific algorithm’s implementation into different
parts.

In this way, a client using the algorithm may choose a specific implementation,
while the general algorithm workflow remains the same. In other words, the
abstract specification of the class does not depend on the specific
implementation of the derived class, but specific implementation must adhere to
the abstract specification. This is why we call it "Dependency Inversion".

## Motivation

Imagine we are working on a project that generates reports every month. We need
the reports to be generated in different formats (strategies), e.g., in `JSON`
or `Plain Text` formats. But things vary over time, and we don't know what kind
of requirement we may get in the future. For example, we may need to generate
our report in a completely new format, or just modify one of the existing
formats.

## Example

In this example our invariants (or abstractions) are `Formatter` and `Report`,
while `Text` and `Json` are our strategy structs. These strategies have to
implement the `Formatter` trait.

```rust
use std::collections::HashMap;

type Data = HashMap<String, u32>;

trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

struct Report;

impl Report {
    // Write should be used but we kept it as String to ignore error handling
    fn generate<T: Formatter>(g: T, s: &mut String) {
        // backend operations...
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        // generate report
        g.format(&data, s);
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{k} {v}\n");
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop(); // remove extra , at the end
        }
        buf.push(']');
    }
}

fn main() {
    let mut s = String::from("");
    Report::generate(Text, &mut s);
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear(); // reuse the same buffer
    Report::generate(Json, &mut s);
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
```

## Advantages

The main advantage is separation of concerns. For example, in this case `Report`
does not know anything about specific implementations of `Json` and `Text`,
whereas the output implementations does not care about how data is preprocessed,
stored, and fetched. The only thing they have to know is a specific trait to
implement and its method defining the concrete algorithm implementation
processing the result, i.e., `Formatter` and `format(...)`.

## Disadvantages

For each strategy there must be implemented at least one module, so number of
modules increases with number of strategies. If there are many strategies to
choose from then users have to know how strategies differ from one another.

## Discussion

In the previous example all strategies are implemented in a single file. Ways of
providing different strategies includes:

- All in one file (as shown in this example, similar to being separated as
  modules)
- Separated as modules, E.g. `formatter::json` module, `formatter::text` module
- Use compiler feature flags, E.g. `json` feature, `text` feature
- Separated as crates, E.g. `json` crate, `text` crate

Serde crate is a good example of the `Strategy` pattern in action. Serde allows
[full customization](https://serde.rs/custom-serialization.html) of the
serialization behavior by manually implementing `Serialize` and `Deserialize`
traits for our type. For example, we could easily swap `serde_json` with
`serde_cbor` since they expose similar methods. Having this makes the helper
crate `serde_transcode` much more useful and ergonomic.

However, we don't need to use traits in order to design this pattern in Rust.

The following toy example demonstrates the idea of the Strategy pattern using
Rust `closures`:

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

In fact, Rust already uses this idea for `Options`'s `map` method:

```rust
fn main() {
    let val = Some("Rust");

    let len_strategy = |s: &str| s.len();
    assert_eq!(4, val.map(len_strategy).unwrap());

    let first_byte_strategy = |s: &str| s.bytes().next().unwrap();
    assert_eq!(82, val.map(first_byte_strategy).unwrap());
}
```

## See also

- [Strategy Pattern](https://en.wikipedia.org/wiki/Strategy_pattern)
- [Dependency Injection](https://en.wikipedia.org/wiki/Dependency_injection)
- [Policy Based Design](https://en.wikipedia.org/wiki/Modern_C++_Design#Policy-based_design)
- [Implementing a TCP server for Space Applications in Rust using the Strategy Pattern](https://web.archive.org/web/20231003171500/https://robamu.github.io/posts/rust-strategy-pattern/)
