# Design Patterns

[Design patterns](https://en.wikipedia.org/wiki/Software_design_pattern) are
"general reusable solutions to a commonly occurring problem within a given
context in software design". Design patterns are a great way to describe the
culture of a programming language. Design patterns are very language-specific -
what is a pattern in one language may be unnecessary in another due to a
language feature, or impossible to express due to a missing feature.

If overused, design patterns can add unnecessary complexity to programs.
However, they are a great way to share intermediate and advanced level knowledge
about a programming language.

## Design patterns in Rust

Rust has many unique features. These features give us great benefit by removing
whole classes of problems. Some of them are also patterns that are _unique_ to Rust.

## YAGNI

YAGNI is an acronym that stands for `You Aren't Going to Need It`.
It's a vital software design principle to apply as you write code.

> The best code I ever wrote was code I never wrote.

If we apply YAGNI to design patterns, we see that the features of Rust allow us to
throw out many patterns. For instance, there is no need for the [strategy pattern](https://en.wikipedia.org/wiki/Strategy_pattern)
in Rust because we can just use [traits](https://doc.rust-lang.org/book/traits.html).
