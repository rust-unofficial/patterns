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

Design patterns are a collection of reusable and tested solutions to
recurring problems in engineering. They make our software more modular,
maintainable, and extensible. Moreover, these patterns provide a common
language for developers, making them an excellent tool for effective
communication when problem-solving in teams.

## Design patterns in Rust

Rust is not object-oriented, and the combination of all its characteristics,
such as functional elements, a strong type system, and the borrow checker,
makes it unique.
Because of this, Rust design patterns vary with respect to other
traditional object-oriented programming languages.
That's why we decided to write this book. We hope you enjoy reading it!
The book is divided in three main chapters:

- [Idioms](./idioms/index.md): guidelines to follow when coding.
  They are the social norms of the community.
  You should break them only if you have a good reason for it.
- [Design patterns](./patterns/index.md): methods to solve common problems
  when coding.
- [Anti-patterns](./anti_patterns/index.md): methods to solve common problems
  when coding.
  However, while design patterns give us benefits,
  anti-patterns create more problems.
