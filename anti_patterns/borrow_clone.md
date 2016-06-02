# Clone to satisfy the borrow checker

## Description

The borrow checker prevents rust users from developing otherwise unsafe code by
ensuring that either: only one mutable reference exists, or potentially many but
all immutable references exist. If the code written does not hold true to these 
conditions, this anti-pattern arises when the developer resolves the compiler
error by cloning the variable.


## Example

```rust fn main() {
    // define any variable
    let mut x = 5;
    
    // Borrow `x` -- but clone it first
    let y = &mut (x.clone()); 

    // perform some action on the borrow to prevent rust from optimizing this
    //out of existence
    *y += 1;

    // without the x.clone() two lines prior, this line would fail on compile as
    // x has been borrowed
    // thanks to x.clone(), x was never borrowed, and this line will run.
    println!("{}", x);
}
```


## Motivation

It is tempting, particularly for beginners, to use this pattern to resolve
confusing issues with the borrow checker. However, there are serious
consequences. Using .clone() causes a copy of the data to be made. Any changes
between the two are not synchronized -- as if two completely separate variables
exist.

There are special cases -- `Rc<T>` is designed to handle clones intelligently.
It internally manages exactly one copy of the data, and cloning it will only
clone the reference.

In general, clones should be deliberate, with full understanding of the
consequences. If a clone is used to make a borrow checker error disappear,
that's a good indication this anti-pattern may be in use.

If an unnecessary clone is suspected, The Rust Book's chapter on Ownership
should be understood fully before assessing whether the clone is required or not.


## See also

[The Rust Book: Ownership, which describes how the borrow checker behaves](https://doc.rust-lang.org/book/ownership.html)
[Rc<T> documentation, which handles .clone() intelligently](http://doc.rust-lang.org/std/rc/)
