# `mem::replace` to keep owned values in changed enums

## Description

Say we have a `&mut MyEnum` which has (at least) two variants,
`A { name: String, x: u8 }` and `B { name: String }`. Now we want to change
`MyEnum::A` to a `B` if `x` is zero, while keeping `MyEnum::B` intact.

We can do this without cloning the `name`.

## Example

```rust
use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String }
}

fn a_to_b(e: &mut MyEnum) {

    // we mutably borrow `e` here. This precludes us from changing it directly
    // as in `*e = ...`, because the borrow checker won't allow it. Therefore
    // the assignment to `e` must be outside the `if let` clause. 
    *e = if let MyEnum::A { ref mut name, x: 0 } = *e {
    
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will 
        // be assigned to `*e`, because it is the result of the `if let` expression).
        MyEnum::B { name: mem::replace(name, String::new()) }
        
    // In all other cases, we return immediately, thus skipping the assignment
    } else { return }
}
```

This also works with more variants:

```Rust
use std::mem;

enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D
}

fn swizzle(e: &mut MultiVariateEnum) {
    use self::MultiVariateEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B { name: mem::replace(name, String::new()) },
        B { ref mut name } => A { name: mem::replace(name, String::new()) },
        C => D,
        D => C
    }
}
```

## Motivation

When working with enums, we may want to change an enum value in place, perhaps
to another variant. This is usually done in two phases to keep the borrow
checker happy. In the first phase, we observe the existing value and look at
its parts to decide what to do next. In the second phase we may conditionally
change the value (as in the example above).

The borrow checker won't allow us to take out `name` of the enum (because
*something* must be there. We could of course `.clone()` name and put the clone
into our `MyEnum::B`, but that would be an instance of the [Clone to satisfy
the borrow checker] antipattern. Anyway, we can avoid the extra allocation by
changing `e` with only a mutable borrow.

`mem::replace` lets us swap out the value, replacing it with something else. In
this case, we put in an empty `String`, which does not need to allocate. As a
result, we get the original `name` *as an owned value*. We can then wrap this in
another enum.

Note, however, that if we are using an `Option` and want to replace its
value with a `None`, `Option`’s `take()` method provides a shorter and
more idiomatic alternative.


## Advantages

Look ma, no allocation! Also you may feel like Indiana Jones while doing it.

## Disadvantages

This gets a bit wordy. Getting it wrong repeatedly will make you hate the
borrow checker. The compiler may fail to optimize away the double store,
resulting in reduced performance as opposed to what you'd do in unsafe
languages.

## Discussion

This pattern is only of interest in Rust. In GC'd languages, you'd take the
reference to the value by default (and the GC would keep track of refs), and in
other low-level languages like C you'd simply alias the pointer and fix things
later.

However, in Rust, we have to do a little more work to do this. An owned value
may only have one owner, so to take it out, we need to put something back in –
like Indiana Jones, replacing the artifact with a bag of sand.


## See also

This gets rid of the [Clone to satisfy the borrow checker] antipattern in a
specific case.

[Clone to satisfy the borrow checker](TODO: Hinges on PR #23)
