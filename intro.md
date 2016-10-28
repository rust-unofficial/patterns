# Introduction

When developing programs, we have to solve many problems. A program can be viewed as a solution to a problem. It can also be viewed as a collection of solutions to many different problems. All of these solutions work together to solve a bigger problem.

There are many problems that share the same form. While the details are different, since they have the same form they can be solved using the same methods. [Design patterns](#design-patterns) are methods to solve common problems when writing software. [Anti-patterns](#anti-patterns) are methods to solve these same common problems. However, while design patterns give us benefits, anti-patterns create more problems. [Idioms](#idioms) are guidelines to follow when coding. They are social norms of the community. You can break them, but if you do you should have a good reason for it.

## Design patterns

[Design patterns](https://en.wikipedia.org/wiki/Software_design_pattern) are "general reusable solutions to a commonly occurring problem within a given context in software design". Design patterns are a great way to describe some of the culture and 'tribal knowledge' of programming in a language. Design patterns are very language-specific - what is a pattern in one language may be unnecessary in another due to a language feature, or impossible to express due to a missing feature.

If overused, design patterns can add unnecessary complexity to programs (e.g., see `AbstractSingletonProxyFactoryBean`). However, I think they are a great way to share intermediate and advanced level knowledge about a programming language.

## Anti-patterns

[Anti-patterns](https://en.wikipedia.org/wiki/Anti-pattern) are solutions to a "recurring problem that is usually ineffective and risks being highly counterproductive". Just as valuable as the knowledge of how to do a thing is how not to do it. Anti-patterns give us great counter-examples to consider relative to design patterns. Anti-patterns are not confined to code. A process can be an anti-pattern.

## Idioms

Idioms are commonly used styles and patterns largely agreed upon by a community. They are guidelines. Writing idiomatic code allows other developers to understand what is happening because they are familiar with the form that it has.

## Design patterns in Rust

TODO:

- functional elements
- type system
- borrow checker

Rust has many very unique features. These features give us great benefit by removing whole classes of problems.

Rust is an imperative language, but it follows many functional programming paradigms. One of the biggest hurdles to understanding functional programs when coming from an imperative background is the shift in thinking. Imperative programs describe __how__ to do something, whereas declarative programs describe __what__ to do. Let's sum the numbers from 1 to 10 to show this.

### Imperative

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

### Declarative

```rust
println!("{}", (1..11).fold(0, |a, b| a + b));
```

Whoa! This is really different! What's going on here?
