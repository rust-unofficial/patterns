# RAII with guards

## Description

RAII stands for "Resource allocation is initialisation" which is a terrible
name. The essence of the pattern is that resource initialisation is done in the
constructor of an object and finalisation in the destructor. This pattern is
extended in Rust by using an RAII object as a guard of some resource and relying
on the type system to ensure that access is always mediated by the guard object.

## Example

Mutex guards are the classic example of this pattern from the std library (this
is a simplified version of the real implementation):

```rust
struct Mutex<T> {
    // We keep a reference to our data: T here.
    ...
}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
    ...
}

// Locking the mutex is explicit.
impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {
        // Lock the underlying OS mutex.
        ...

        // MutexGuard keeps a reference to self
        MutexGuard { data: self, ... }
    }
}

// Destructor for unlocking the mutex.
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Unlock the underlying OS mutex.
        ...
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

fn main(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // foo is a method on Foo.
    // The borrow checker ensures we can't store a reference to the underlying
    // Foo which will outlive the guard xx.

    // x is unlocked when we exit this function and xx's destructor is executed.
}
```


## Motivation

Where a resource must be finalised after use, RAII can be used to do this
finalisation. If it is an error to access that resource after finalisation, then
this pattern can be used to prevent such errors.


## Advantages

Prevents errors where a resource is not finalised and where a resource is used
after finalisation.


## Discussion

RAII is a useful pattern for ensuring resources are properly deallocated or
finalised. We can make use of the borrow checker in Rust to statically prevent
errors stemming from using resources after finalisation takes place.

The core aim of the borrow checker is to ensure that references to data do not
outlive that data. The RAII guard pattern works because the guard object
contains a reference to the underlying resource and only exposes such
references. Rust ensures that the guard cannot outlive the underlying resource
and that references to the resource mediated by the guard cannot outlive the
guard. To see how this works it is helpful to examine the signature of `deref`
without lifetime elision:

```rust
fn deref<'a>(&'a self) -> &'a T { ... }
```

The returned reference to the resource has the same lifetime as `self` (`'a`).
The borrow checker therefore ensures that the lifetime of the reference to `T`
is shorter than the lifetime of `self`.

Note that implementing `Deref` is not a core part of this pattern, it only makes
using the guard object more ergonomic. Implementing a `get` method on the guard
works just as well.



## See also

[Finalisation in destructors idiom](../idioms/dtor-finally.md)

RAII is a common pattern in C++: [cppreference.com](http://en.cppreference.com/w/cpp/language/raii),
[wikipedia](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization).

[Style guide enty](http://doc.rust-lang.org/stable/style/ownership/raii.html)
(currently just a placeholder).
