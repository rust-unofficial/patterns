# Introduction

## Participation

If you are interested in contributing to this book, check out the
[contribution guidelines](https://github.com/rust-unofficial/patterns/blob/master/CONTRIBUTING.md).

## Design patterns

In software development, we often come across problems that share 
similarities regardless of the environment they appear in. Although the 
implementation details are crucial to solve the task at hand, we may 
abstract from these particularities to find the common practices that 
are generically applicable.

Design patterns are a collection of reusable tried and tested solutions to 
recurring problems in engineering. They make our software more modular, 
maintainable, and extensible. Moreover, these patterns provide a common 
language for developers to use, making them an excellent tool for effective 
communication when problem-solving in teams.

## Design patterns in Rust

There are many problems that share the same form.
Due to the fact that Rust is not object-oriented, design patterns vary with
respect to other object-oriented programming languages.
While the details are different, since they have the same form they can be
solved using the same fundamental methods:

- [Design patterns](./patterns/index.md) are methods to solve common problems
  when writing software.
- [Anti-patterns](./anti_patterns/index.md) are methods to solve these same
  common problems. However, while design patterns give us benefits,
  anti-patterns create more problems.
- [Idioms](./idioms/index.md) are guidelines to follow when coding.
  They are social norms of the community.
  You can break them, but if you do you should have a good reason for it.

TODO: Mention why Rust is a bit special - functional elements, type system,
borrow checker
