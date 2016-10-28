# Introduction

## Design patterns

[Design patterns](https://en.wikipedia.org/wiki/Software_design_pattern) are "general reusable solutions to a commonly occurring problem within a given context in software design". Design patterns are a great way to describe some of the culture and 'tribal knowledge' of programming in a language. Design patterns are very language-specific - what is a pattern in one language may be unnecessary in another due to a language feature, or impossible to express due to a missing feature.

If overused, design patterns can add unnecessary complexity to programs (e.g., see `AbstractSingletonProxyFactoryBean`). However, I think they are a great way to share intermediate and advanced level knowledge about a programming language.

So for these reasons, I have been thinking about design patterns in Rust, and in particular those that are unique to Rust or more common in Rust than other languages. I have started a [pattern catalogue for Rust](https://github.com/rust-unofficial/patterns/blob/master/README.md). It covers idioms (small, simpler design patterns), design patterns, and anti-patterns (patterns which you should strive to avoid). There's not a lot there at the moment. I hope to improve it over the next few months. I would love some help with that - if you like writing documentation and fancy describing some design patterns, please send a PR! There is a list of design patterns which need a description in the contents. If you know of other design patterns in Rust and want to add them, or want to improve some of the existing descriptions, that would be awesome.

## Anti-patterns

[Anti-patterns](https://en.wikipedia.org/wiki/Anti-pattern) are solutions to a "recurring problem that is usually ineffective and risks being highly counterproductive". Just as valuable as the knowledge of how to do a thing is how not to do it. Anti-patterns give us great counter-examples to consider relative to design patterns. Anti-patterns are not confined to code. A process can be an anti-pattern.

## Idioms

Idioms are commonly used styles and patterns largely agreed upon by a community. They are guidelines. Idioms do not _have_ to be used, but if you are not writing idiomatic code you should have a good reason for it. Writing idiomatic code allows other developers to understand what is happening because they are familiar with the form that it has.

## Design patterns in Rust

Why Rust is a bit special - functional elements, type system - borrow checker
