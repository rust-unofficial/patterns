# Design patterns

[Design patterns](https://en.wikipedia.org/wiki/Software_design_pattern) are "general reusable solutions to a commonly occurring problem within a given context in software design". Design patterns are a great way to describe some of the culture and 'tribal knowledge' of programming in a language. Design patterns are very language-specific - what is a pattern in one language may be unnecessary in another due to a language feature, or impossible to express due to a missing feature.

If overused, design patterns can add unnecessary complexity to programs. However, they are a great way to share intermediate and advanced level knowledge about a programming language.

# Design patterns in Rust

Rust has many very unique features. These features give us great benefit by removing whole classes of problems.

# Functional usage of Rust

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
