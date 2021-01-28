# Interpreter

## Description

If a problem occurs very often and requires long and repetitive steps to solve
it, then the problem instances might be expressed in a simple language and an
interpreter object could solve it by interpreting the sentences written in this
simple language. Basically, for any kind of problems we define a domain language,
then define a grammar for this language and design interpreter solving problem
instances.

## Motivation

Our goal is translate simple mathematical expressions into
postfix expressions (or [Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation))
For simplicity, our expressions consists of ten digits `0`,...,`9`,
and two operations `+, -` and a pair of parenthesis.
For example, expression `2 + 4` could is translated into `2 4 +`.

## Context Free Grammars

In order to understand the Interpreter pattern you need to know a little bit
about formal language theory. If you already know what a grammar is feel free
to skip this section, otherwise fasten your seat belts.

Basicaly, a [Context Free Grammar](https://en.wikipedia.org/wiki/Context-free_grammar)
(CFG) defines a set of rules that describe all possible strings over some finite
set of symbols, formally called alphabet. These strings usually have some well
defined structure. For example, strings of all balanced parentheses:

```ignore
(), (()), ()(), ()(()), ..
```

Formally a CFG consists of terminal symbols, nonterminal symbols, a start (nonterminal)
symbol, and production rules.

In the previous example we have two terminal symbols `(` and `)`. We define a
single nonterminal (at the same time the start) symbol `S` and production rules
as:

```ignore
S -> SS
S -> (S)
S -> ()
```

So, in order to generate a string with balanced parentheses we repeatedly apply
production rules. For example, to derive `()()` we apply the following rules:

```ignore
S -> SS -> ()(S) -> ()()
```

## Context Free Grammar for our problem

Recal that our task is translate infix expressions into postfex ones.
Let's define a context free grammar for a set of expressions over
`0,...,9, +,-`, where

- terminal symbols: `0,...,9, +,-`
- nonterminal symbols: `exp, term`
- start symbol is `exp`
- and the following are production rules

```ignore
exp -> exp + term
exp -> exp - term
exp -> term
term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

In fact this grammar should be further transformed depending on what we are going
to do with the grammar. For example, we might need to remove left recursion.
For more details please see [Compilers:Principles,Techniques, and Tools](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
(aka Dragon Book).

## Solution 1

Our first approach is a standard one, simple implementation of a recursive descent
parser. The code panics when an expression is syntactically wrong
(for exmple `2-34` or `2+5-` are wrong according to the grammar definition).

```rust
struct Interpreter<'a> {
    it: std::str::Chars<'a>,
}
impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a String) -> Self {
        Self { it: infix.chars() }
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.exp(out);
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    fn exp(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{}'", op);
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected symbol '{}'", ch),
            None => panic!("Unexpected end of string"),
        }
    }
}

pub fn main() {
    let mut infix = String::from("2+3");
    let mut postfix = String::new();
    let mut intr = Interpreter::new(&infix);
    intr.interpret(&mut postfix);
    assert_eq!(postfix, "23+");

    infix = String::from("1-2+3-4");
    postfix = String::new();
    intr = Interpreter::new(&infix);
    intr.interpret(&mut postfix);

    assert_eq!(postfix, "12-3+4-");
}
```

## Solution 2

The second approach is using Rust's `macro_rules!`. We simply define rules and
leave the rest to Rust's interpretation of these rules wich converts a given
expression into corresponding assembly code.

```rust
fn print_op(op: &str) {
    println!("pop ebx");
    println!("pop eax");
    println!("{} eax,ebx", op);
    println!("push eax");
}

macro_rules! term {
    ($x:tt * $($tail:tt)+) => {
        term!($x);
        term!($($tail)+);
        print_op("mul");
    };

    ($x:tt / $($tail:tt)+) => {
        term!($x);
        term!($($tail)+);
        print_op("div");
    };

    ($x:ident) => {
        println!("push {}", $x);
    };
    ($x:literal) => {
        println!("push {}", $x);
    };
    (($($x:tt)*)) => {
        to_asm!($($x)*);
    };
}

macro_rules! to_asm {
    ($x:tt + $($tail:tt)+) => {
        to_asm!($x);
        to_asm!($($tail)+);
        print_op("add");
    };

    ($x:tt - $($tail:tt)+) => {
        to_asm!($x);
        to_asm!($($tail)+);
        print_op("sub");
    };

    ($($x:tt)*) => {
        term!($($x)*);
    };
}

fn main() {
    let a = 3;
    to_asm!((2 * 3) - 5);
    println!("-------------------");
    to_asm!(2 * (a - 5));
}
```

Output

```ignore
push 2
push 3
pop ebx
pop eax
mul eax,ebx
push eax
push 5
pop ebx
pop eax
sub eax,ebx
push eax
-------------------
push 2
push 3
push 5
pop ebx
pop eax
sub eax,ebx
push eax
pop ebx
pop eax
mul eax,ebx
push eax
```

## See also

- [Interpreter pattern](https://en.wikipedia.org/wiki/Interpreter_pattern)
- [macro_rules!](https://doc.rust-lang.org/rust-by-example/macros.html)
- [Contex free grammar](https://en.wikipedia.org/wiki/Context-free_grammar)
