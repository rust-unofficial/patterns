# Interpreter

## Description

If a problem occurs very often and requires long and repetitive steps to solve
it, then the problem instances might be expressed in a simple language and an
interpreter object could solve it by interpreting the sentences written in this
simple language.

Basically, for any kind of problems we define:

- A
  [domain specific language](https://en.wikipedia.org/wiki/Domain-specific_language),
- A grammar for this language,
- An interpreter that solves the problem instances.

## Motivation

Our goal is to translate simple mathematical expressions into postfix
expressions (or
[Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation))
For simplicity, our expressions consist of ten digits `0`, ..., `9` and two
operations `+`, `-`. For example, the expression `2 + 4` is translated into
`2 4 +`.

## Context Free Grammar for our problem

Our task is translating infix expressions into postfix ones. Let's define a
context free grammar for a set of infix expressions over `0`, ..., `9`, `+`, and
`-`, where:

- Terminal symbols: `0`, `...`, `9`, `+`, `-`
- Non-terminal symbols: `exp`, `term`
- Start symbol is `exp`
- And the following are production rules

```ignore
exp -> exp + term
exp -> exp - term
exp -> term
term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

**NOTE:** This grammar should be further transformed depending on what we are
going to do with it. For example, we might need to remove left recursion. For
more details please see
[Compilers: Principles,Techniques, and Tools](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
(aka Dragon Book).

## Solution

We simply implement a recursive descent parser. For simplicity's sake, the code
panics when an expression is syntactically wrong (for example `2-34` or `2+5-`
are wrong according to the grammar definition).

```rust
pub struct Interpreter<'a> {
    it: std::str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{op}'");
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected symbol '{ch}'"),
            None => panic!("Unexpected end of string"),
        }
    }
}

pub fn main() {
    let mut intr = Interpreter::new("2+3");
    let mut postfix = String::new();
    intr.interpret(&mut postfix);
    assert_eq!(postfix, "23+");

    intr = Interpreter::new("1-2+3-4");
    postfix.clear();
    intr.interpret(&mut postfix);
    assert_eq!(postfix, "12-3+4-");
}
```

## Discussion

There may be a wrong perception that the Interpreter design pattern is about
design grammars for formal languages and implementation of parsers for these
grammars. In fact, this pattern is about expressing problem instances in a more
specific way and implementing functions/classes/structs that solve these problem
instances. Rust language has `macro_rules!` that allow us to define special
syntax and rules on how to expand this syntax into source code.

In the following example we create a simple `macro_rules!` that computes
[Euclidean length](https://en.wikipedia.org/wiki/Euclidean_distance) of `n`
dimensional vectors. Writing `norm!(x,1,2)` might be easier to express and more
efficient than packing `x,1,2` into a `Vec` and calling a function computing the
length.

```rust
macro_rules! norm {
    ($($element:expr),*) => {
        {
            let mut n = 0.0;
            $(
                n += ($element as f64)*($element as f64);
            )*
            n.sqrt()
        }
    };
}

fn main() {
    let x = -3f64;
    let y = 4f64;

    assert_eq!(3f64, norm!(x));
    assert_eq!(5f64, norm!(x, y));
    assert_eq!(0f64, norm!(0, 0, 0));
    assert_eq!(1f64, norm!(0.5, -0.5, 0.5, -0.5));
}
```

## See also

- [Interpreter pattern](https://en.wikipedia.org/wiki/Interpreter_pattern)
- [Context free grammar](https://en.wikipedia.org/wiki/Context-free_grammar)
- [macro_rules!](https://doc.rust-lang.org/rust-by-example/macros.html)
