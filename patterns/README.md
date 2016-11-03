# Design patterns

[Design patterns](https://en.wikipedia.org/wiki/Software_design_pattern) are "general reusable solutions to a commonly occurring problem within a given context in software design". Design patterns are a great way to describe some of the culture and 'tribal knowledge' of programming in a language. Design patterns are very language-specific - what is a pattern in one language may be unnecessary in another due to a language feature, or impossible to express due to a missing feature.

If overused, design patterns can add unnecessary complexity to programs (e.g., see `AbstractSingletonProxyFactoryBean`). However, they are a great way to share intermediate and advanced level knowledge about a programming language.

## Creational design patterns

- Abstract Factory
	- Creates an instance of several families of classes
- Builder
	- Separates object construction from its representation
- Factory Method
	- Creates an instance of several derived classes
- Object Pool
	- Avoid expensive acquisition and release of resources by recycling objects that are no longer in use
- Prototype
	- A fully initialized instance to be copied or cloned
- Singleton
	- A class of which only a single instance can exist

## Structural design patterns

- Adapter
	- Match interfaces of different classes
- Bridge
	- Separates an object's interface from its implementation
- Composite
	- A tree structure of simple and composite objects
- Decorator
	- Add responsibilities to objects dynamically
- Facade
	- A single class that represents an entire subsystem
- Flyweight
	- A fine-grained instance used for efficient sharing
- Private Class Data
	- Restricts accessor/mutator access
- Proxy
	- An object representing another object

## Behavioral design patterns

- Chain of responsibility
	- A way of passing a request between a chain of objects
- Command
	- Encapsulate a command request as an object
- Interpreter
	- A way to include language elements in a program
- Iterator
	- Sequentially access the elements of a collection
- Mediator
	- Defines simplified communication between classes
- Memento
	- Capture and restore an object's internal state
- Null Object
	- Designed to act as a default value of an object
- Observer
	- A way of notifying change to a number of classes
- State
	- Alter an object's behavior when its state changes
- Strategy
	- Encapsulates and algorithm inside a class
- Template Method
	- Defer the exact steps of an algorithm to a subclass
- Visitor
	- Defines a new operation to a class without change

## Criticism

- Targets the wrong problem
- Lacks formal foundations
- Leads to inefficient solutions
- Does not differ significantly from other abstractions

# Design patterns in Rust

Rust has many very unique features. These features give us great benefit by removing whole classes of problems.

## Traditional Design Patterns

### Strategy

The Strategy pattern can be implemented in a straight-forward way using traits. Using traits will allow the desired behavior without the need to store an implementation.

```rust
trait TranportationToAirport {
	fn go_to_airport(&self);
}

struct Bus;
struct Car;
struct Taxi;

impl TranportationToAirport for Bus {
	fn go_to_airport(&self) {
		println!("Riding the bus");
	}
}

impl TranportationToAirport for Car {
	fn go_to_airport(&self) {
		println!("Driving a car");
	}
}

impl TranportationToAirport for Taxi {
	fn go_to_airport(&self) {
		println!("Riding in a taxi");
	}
}

fn main() {
	let bus = Bus;
	let car = Car;
	let taxi = Taxi;
	bus.go_to_airport();
	car.go_to_airport();
	taxi.go_to_airport();
}
```

### Observer

```rust

```

Rust is an imperative language, but it follows many functional programming paradigms. One of the biggest hurdles to understanding functional programs when coming from an imperative background is the shift in thinking. Imperative programs describe __how__ to do something, whereas declarative programs describe __what__ to do. Let's sum the numbers from 1 to 10 to show this.

## Imperative

```rust
let mut sum = 0;
for i in 1..11 {
	sum += i;
}
println!("{}", sum);
```

With imperative programs, we have to play compiler to see what is happening. Here, we start with a `sum` of `0`. Next, we iterate through the range from 1 to 10. Each time through the loop, we add the corresponding value in the range. Then we print it out.

| `i` | `sum` |
| --- | --- |
| 1 | 1 |
| 2 | 3 |
| 3 | 6 |
| 4 | 10 |
| 5 | 15 |
| 6 | 21 |
| 7 | 28 |
| 8 | 36 |
| 9 | 45 |
| 10 | 55 |

This is how most of us start out programming. We learn that a program is a set of steps.

## Declarative

```rust
println!("{}", (1..11).fold(0, |a, b| a + b));
```

Whoa! This is really different! What's going on here? Remember that with declarative programs we are describing __what__ to do, rather than __how__ to do it. `fold` is a function that [composes](https://en.wikipedia.org/wiki/Function_composition) functions. The name is a convention from Haskell.

Here, we are composing functions of addition (this closure: `|a, b| a + b)`) with a range from 1 to 10. The `0` is the starting point, so `a` is `0` at first. `b` is the first element of the range, `1`. `0 + 1 = 1` is the result. So now we `fold` again, with `a = 1`, `b = 2` and so `1 + 2 = 3` is the next result. This process continues until we get to the last element in the range, `10`.

| `a` | `b` | result |
| --- | --- | --- |
| 0 | 1 | 1 |
| 1 | 2 | 3 |
| 3 | 3 | 6 |
| 6 | 4 | 10 |
| 10 | 5 | 15 |
| 15 | 6 | 21 |
| 21 | 7 | 28 |
| 28 | 8 | 36 |
| 36 | 9 | 45 |
| 45 | 10 | 55 |

## Types

Rust has strong static types. This can be very different than what you are used to if you are coming from a loosely-typed language. Don't worry, though. Once you get used to them, you'll find the types actually make your life easier. Why? Because you are making implicit assumptions explicit.

## Ownership

Ownership is arguably the central concept of Rust. The ownership system is __the__ reason for the safety guarantees in Rust.
