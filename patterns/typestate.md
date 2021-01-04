# The Typestate Pattern

## Description

(from http://cliffle.com/blog/rust-typestate/):

The typestate pattern is an API design pattern that encodes information about an object's run-time state in its compile-time type. In particular, an API using the typestate pattern will have:

* Operations on an object (such as methods or functions) that are only available when the object is in certain states,

* A way of encoding these states at the type level, such that attempts to use the operations in the wrong state fail to compile,

* State transition operations (methods or functions) that change the type-level state of objects in addition to, or instead of, changing run-time dynamic state, such that the operations in the previous state are no longer possible.

## Example

This is one example of how to implement the pattern for a simple type that needs to be initialized or otherwise prepared before use 
(that is, it has two states: (), and Ready). More states and operations may be added to implement a more complex state machine.

See http://cliffle.com/blog/rust-typestate/ for more in-depth examples and techniques.

```rust

use std::marker::PhantomData;

pub struct Ready;

pub struct Thing<S = ()> {
     // tracks state type info at compile time, optimized out for runtime.
    marker: PhantomData<S>
}

// Private constructor to internally control what state the struct is in.
fn state_constructor<S>() -> Thing<S> {
    Thing { marker: PhantomData }
}

// Operations in our default state ()
impl Thing {
    pub fn new() -> Self {
        Self { marker: PhantomData }
    }

    // Consumes the struct to return one with a new type state
    pub fn get_ready(self) -> Thing<Ready> {
        state_constructor::<Ready>()
    }
}

// Operations available in any state
impl<S> Thing<S> {
    pub fn do_any_time(&self) {
        println!("We can do this function whenever");
    }
}

// We can only use this function when ready
pub fn do_only_when_ready(_: Thing<Ready>) {
    println!("We can only do this when we are Ready")
}

fn main() {
    let thing = Thing::new();

    // Not ready yet
    thing.do_any_time();
    // do_only_when_ready(thing); // this won't compile

    // Transition to Ready
    let ready = thing.get_ready();

    // Now we're ready
    ready.do_any_time();
    do_only_when_ready(ready);
}
```


## Motivation

You are modelling a system that functions as a state machine, and want to ensure at compile-time that invalid states never occur in any runtime scenario.

## Advantages

(again, from http://cliffle.com/blog/rust-typestate/)

* It moves certain types of errors from run-time to compile-time, giving programmers faster feedback.
* It interacts nicely with IDEs, which can avoid suggesting operations that are illegal in a certain state.
* It can eliminate run-time checks, making code faster/smaller.

## Disadvantages

* It can add some verbosity and complexity to the code.
* Implementing it for complex structs can be difficult.
* It can make compiler error messages very hard to understand.

## See also

http://cliffle.com/blog/rust-typestate/
