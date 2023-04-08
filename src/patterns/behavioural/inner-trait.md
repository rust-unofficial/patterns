# Inner Trait

## Description

It is possible to define a private trait that implements all the
methods of a public trait and also includes some private functions. This pattern
can be used to provide additional functionality to the implementation of a
public trait while keeping the private methods hidden from the public API.

## Example

This example demonstrate how a public trait `Car` can be implemented and include
extra private methods using a auxiliary private trait `InnerCar`.

```rust,ignore
// trait that is public and part of the API
pub trait Car {
    fn get_speed(&self) -> f64;
    fn accelerate(&mut self, duration: f64);
    fn brake(&mut self, force: f64);
}
//not public
mod inner_lib {
    // trait that is only accessible to this crate
    pub(crate) trait InnerCar {
        fn get_speed(&self) -> f64;
        //private
        fn set_speed(&mut self, new_speed: f64);
        //private
        fn get_acceleration(&self) -> f64;
        fn accelerate(&mut self, duration: f64) {
            self.set_speed(
                self.get_speed() + (self.get_acceleration() * duration)
            );
        }
        fn brake(&mut self, force: f64) {
            self.set_speed(
                self.get_speed() - (force * self.get_acceleration())
            );
        }
    }
    //Auto implement Car for all InnerCar, by forwarding the Car trait to the
    //InnerCar implementation
    impl<T: InnerCar> crate::Car for T {
        fn get_speed(&self) -> f64 {
            <Self as InnerCar>::get_speed(self)
        }
        fn accelerate(&mut self, duration: f64) {
            <Self as InnerCar>::accelerate(self, duration)
        }
        fn brake(&mut self, force: f64) {
            <Self as InnerCar>::brake(self, force)
        }
    }
}

#[derive(Default)]
pub struct Car1(f64);
//is not necessary to implement `accelerate` and `brake`, as inner_trait can do that.
impl inner_lib::InnerCar for Car1 {
    fn get_speed(&self) -> f64 {self.0}
    fn set_speed(&mut self, new_speed: f64) {self.0 = new_speed}
    fn get_acceleration(&self) -> f64 {0.10}
}
//more Car implementations...
```

## Motivation

This pattern allows developers to provide additional functionality to the
implementation of a public trait without exposing that functionality as part of
the public API. By using a private trait, developers can also improve the
reusability of their code, since the private functionality can be reused across
multiple implementations of the public trait.

## Advantages

- Provides hidden functionality while keeping the private methods from the API.
- Improves modularity by separating public and private functionality.
- Increases code reusability, since the private functionality can be reused.

## Disadvantages

- Can be harder to understand if the private trait are not well documented.
- Can lead to tight coupling between the public and private functionality.

## Discussion

This pattern is very similar to the concept of "interfaces" and "abstract types"
with public/private methods in object-oriented programming.

In object-oriented programming (OOP), private methods can be used to encapsulate
implementation details within an interface, while public methods exposes
functionality.

In rust there is the public/private trait analogous, the private trait
implementing the hidden functionalities, while the public trait exposes
functionality.

## See also

Wikipedia [OOP Interface](https://en.wikipedia.org/wiki/Interface_%28object-oriented_programming%29).

Blog post from [Predrag](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/)
about sealed, private and other patterns for traits.
