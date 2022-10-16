# Resource management with OBRM

## Description

"Ownership Based Resource Management" (OBRM) - also known as ["Resource Acquisition is Initialisation" (RAII)][wikipedia] - is an idom meant to make handling resources easier and less error-prone.

In essence it means that an object serves as proxy for a resource, to create the object you have to aquire the resource, once that object isn't used anymore - determined by it being unreachable - the resource is released.
It is said the object guards access to the resource.

This idom is supported by the language as it allows to automatically insert calls to the releasing code in the spots where the object becomes unreachable.
The method releasing the resource is generally referred to as destructor, in Rust [drop][Drop::drop] serves that role.

## Example

OBRM is used to manage memory in Rust, determining when to free the memory. 
`Box` and `Rc` are classical examples of that.
But most users will have closer contact with OBRM when managing other aspects.

Mutex guards are the classic example of this pattern from the std library (this
is a simplified version of the real implementation):

```rust,ignore
use std::ops::Deref;

struct Foo {}

struct Mutex<T> {
    // We keep a reference to our data: T here.
    //..
}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
    //..
}

// Locking the mutex is explicit.
impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {
        // Lock the underlying OS mutex.
        //..

        // MutexGuard keeps a reference to self
        MutexGuard {
            data: self,
            //..
        }
    }
}

// Destructor for unlocking the mutex.
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Unlock the underlying OS mutex.
        //..
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // foo is a method on Foo.
    // The borrow checker ensures we can't store a reference to the underlying
    // Foo which will outlive the guard xx.

    // x is unlocked when we exit this function and xx's destructor is executed.
}
```

## Motivation

Often times a user will not need to implement [Drop::drop] themselves but will already be covered by just using the provided OBRM-Objects from the standard library or used crates.

<!-- TODO that feels sluggish -->
But for managing external resources it is often helpful, when communicating with external systems, or of course if implementing your own resources.

## Advantages

Prevents errors where a resource is not finalised and where a resource is used
after finalisation.

## Disadvantages

OBRM ensures correctness with implicit behavior, which isn't visible in the source code (one needs to be aware that said object uses OBRM). It also can be difficult to implement for some complex situations. For example resource aquisition and release in bulk, like in performance critical code. Or code which may not fail in some sections - resource aquisition is often fallible.

OBRM interaction with asyncronous code can also [be unexpected][Documentation of tokios Mutex].

## Discussion

OBRM is a useful pattern for ensuring resources are properly handled. 
The borrow checker in Rust will statically prevent
errors stemming from using resources after the resource has been released.

The core aim of the borrow checker is to ensure that references to data do not
outlive that data. The OBRM guard pattern works because the guard object
acts as a proxy to the underlying resource and enables access only via
references. Rust ensures that the guard cannot outlive the underlying resource
and that references to the resource mediated by the guard cannot outlive the
guard. To see how this works it is helpful to examine the signature of `deref`
without lifetime elision:

```rust,ignore
fn deref<'a>(&'a self) -> &'a T {
    //..
}
```

The returned reference to the resource has the same lifetime as `self` (`'a`).
The borrow checker therefore ensures that the lifetime of the reference to `T`
is shorter than the lifetime of `self`.

Note that implementing `Deref` is not a core part of this pattern, it only makes
using the guard object more ergonomic. Implementing a `get` method on the guard
works just as well.

When compared with RAII in C++, there are a few significant differences:

* while C++ code often interfaces with C code or code in older styles, which doesn't use RAII, Rust was designed without the need to interface with such code, so its far less common to implement OBRM yourself
* C++ doesn't have `Deref` nor a borrow checker, so code using RAII can not archive the same combination of safety and ergonomics
* perhaps most importantly, Rust has different semantics when it comes to moving and copying of values, this will be expanded on below.

C++ has complex rules for copying and moving of values, that Rust managed to simplify while keeping most advantages.
In C++ behavior on a "move" (which is semantically meant to signify passing held resources to the moved-to value) is customizable in its move and move-assignment constructors.
But after a variable has been "moved out of", it must still be accessable in C++.
In Rust, a moved-out-of variable can not be used, only reassigned a new value (this is referred to as "destructive move"), and the behavior on a move is not customizable, instead a move simply copies the bytes of the moved-out value into the moved-into variable, and ensures the semantics of a destructive move.

This massively simplifies creation and management of OBRM Objects compared to C++, where one often has to do a lot more manual management of RAII classes - definition of the `destructor`, the `copy constructor`, the `copy assignment constructor`, the `move constructor` and the `move assignment constructor` all at once -, which is very error prone, and where RAII objects have to have a legal moved-out state, which often makes usage of these classes more problematic.
For example, `unique_ptr`, the C++ equivalent to `Box`, can contain `nullptr`.

Rust also moves values by default, which can be opted out by explicitly calling `Clone::clone` on each assignment, or on a Type level by implementing `Copy`.
It is currently forbidden, and that is expected to continue, to implement `Copy` on a Type that implements `Drop` or contains a Type that implements `Drop`.
This means that resource aquisition in Rust is a lot more explicit than in C++, as it can not happen during a simple assignment as it can in C++.

## See also

[Finalisation in destructors idiom](../../idioms/dtor-finally.md)

RAII is a common pattern in C++: [cppreference.com](http://en.cppreference.com/w/cpp/language/raii),
[wikipedia][wikipedia].

[wikipedia]: https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization

[Rustnomicon entry]: https://doc.rust-lang.org/nomicon/obrm.html

[Drop::drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop

[Documentation of tokios Mutex]: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use

[The Rule of 5/3/0 in C++]: https://en.cppreference.com/w/cpp/language/rule_of_three

Rustdoc to std::marker::Copy explaining why [Copy forbids implementing Drop]: https://doc.rust-lang.org/std/marker/trait.Copy.html#when-cant-my-type-be-copy

[Discussion of Copy: !Drop (highly theoretical)]: https://stackoverflow.com/a/67645936