# Compose structs together for better borrowing

TODO - this is not a very snappy name

## Description

Sometimes a large struct will cause issues with the borrow checker - although
fields can be borrowed independently, sometimes the whole struct ends up being
used at once, preventing other uses. A solution might be to decompose the struct
into several smaller structs. Then compose these together into the original
struct. Then each struct can be borrowed separately and have more flexible
behaviour.

This will often lead to a better design in other ways: applying this design
pattern often reveals smaller units of functionality.


## Example

Here is a contrived example of where the borrow checker foils us in our plan to
use a struct:

```rust
struct A {
    f1: u32,
    f2: u32,
    f3: u32,
}

fn foo(a: &mut A) -> &u32 { &a.f2 }
fn bar(a: &mut A) -> u32 { a.f1 + a.f3 }

fn main(a: &mut A) {
    // x causes a to be borrowed for the rest of the function.
    let x = foo(a);
    // Borrow check error
    let y = bar(a); //~ ERROR: cannot borrow `*a` as mutable more than once at a time
}
```

We can apply this design pattern and refactor `A` into two smaller structs, thus
solving the borrow checking issue:

```rust
// A is now composed of two structs - B and C.
struct A {
    b: B,
    c: C,
}
struct B {
    f2: u32,
}
struct C {
    f1: u32,
    f3: u32,
}

// These functions take a B or C, rather than A.
fn foo(b: &mut B) -> &u32 { &b.f2 }
fn bar(c: &mut C) -> u32 { c.f1 + c.f3 }

fn main(a: &mut A) {
    let x = foo(&mut a.b);
    // Now it's OK!
    let y = bar(&mut a.c);
}
```


## Motivation

Why and where you should use the pattern


## Advantages

Lets you work around limitations in the borrow checker.

Often produces a better design.


## Disadvantages

Leads to more verbose code.

Sometimes, the smaller structs are not good abstractions, and so we end up with
a worse design. That is probably a 'code smell', indicating that the program
should be refactored in some way.


## Discussion

This pattern is not required in languages that don't have a borrow checker, so
in that sense is unique to Rust. However, making smaller units of functionality
often leads to cleaner code: a widely acknowledged principle of software
engineering, independent of the language.

This pattern relies on Rust's borrow checker to be able to borrow fields
independently of each other. In the example, the borrow checker knows that `a.b`
and `a.c` are distinct and can be borrowed independently, it does not try to
borrow all of `a`, which would make this pattern useless.
