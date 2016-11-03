# Design patterns

[Design patterns](https://en.wikipedia.org/wiki/Software_design_pattern) are "general reusable solutions to a commonly occurring problem within a given context in software design". Design patterns are a great way to describe some of the culture and 'tribal knowledge' of programming in a language. Design patterns are very language-specific - what is a pattern in one language may be unnecessary in another due to a language feature, or impossible to express due to a missing feature.

If overused, design patterns can add unnecessary complexity to programs. However, they are a great way to share intermediate and advanced level knowledge about a programming language.

# Design patterns in Rust

Rust has many very unique features. These features give us great benefit by removing whole classes of problems.

# YAGNI

If you're not familiar with it, YAGNI is an acronym that stands for You Aren't Going to Need It. It's an important software principle to apply as you write code.

> The best code I ever wrote was no code at all.

If we apply YAGNI to design patterns, we see that the features of Rust allow us to throw out many patterns. For instance, there is no need for the strategy pattern in Rust because we can just use traits.

## Types

Rust has strong static types. This can be very different than what you are used to if you are coming from a loosely-typed language. Don't worry, though. Once you get used to them, you'll find the types actually make your life easier. Why? Because you are making implicit assumptions explicit.

## Ownership

Ownership is arguably the central concept of Rust. The ownership system is __the__ reason for the safety guarantees in Rust.
