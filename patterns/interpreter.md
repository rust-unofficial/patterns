# Interpreter

## Description

If a problem occurs very often and requires long and repetetive steps to solve it,
then the problem instances might be expressed in a simple
language and inerpreter object could solve it by
interprting the sentences written in this simple languge.
Basically, for any kind of problems we define a domain
language, then define a grammar for this language and
design interpreter solving problem instances.

## Motivation

Imagine thta our work is translating simple mathemtaical expressions into
[assembly language](https://en.wikipedia.org/wiki/Assembly_language)
(more simple and low level programming language).
For simplicity, our expressions consists of ten digits `0`,...,`9`,
four operations `+, -, /, *` and a pair of prenthesis `(, )`.
For example, experssion `2 + 4` could be translated into

```ignore
mov eax, 2
mov ebx, 4
add eax, ebx
```

Our goal is to automate translating into assembly instructions
using the Interpreter design pattern. In other words, we want simply
provide the Interpreter with an expressions and get assembly
language output. For example

```rust, ignore
x.interpret("7+3*(2-1)", &output);
```

We don't claim that output assembly code is exactly what modern compilers generate,
but it at least comuptes the expression correctly.
It is simpy an example of using Interpreter pattern.

## Solution 1

The grammar for a set of espressions over `0,...,9, +,-,*,/,(,)` is

```ignore
exp -> exp + term
exp -> exp - term
exp -> term
term -> term * factor
term -> term / factor
term -> factor
factor -> ( exp )
factor -> 0 | 1| 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

Our first approach is a standard one, simple implementation of
a recursive descent parser.The following code
doesn't have `struct` abstraction in order to keep code short.
The code panics when expression is syntactically wrong
(unbalanced parantheses or missing digit/operator for example).

```rust
fn token(input: &[u8], cur: usize) -> char {
    if cur < input.len() {
        input[cur] as char
    } else {
        '$' // End of line
    }
}

fn single_expr(input: &[u8], cur: &mut usize, out: &mut Vec<String>) {
    expr(input, cur, out);

    let ch = token(input, *cur);
    if ch != '$' {
        panic!("Unexpected symbol '{}', at {}", ch, *cur);
    }
}

fn expr(input: &[u8], cur: &mut usize, out: &mut Vec<String>) {
    term(input, cur, out);

    loop {
        let ch = token(input, *cur);
        if ch == '$' || (ch != '+' && ch != '-') {
            break;
        } else {
            *cur += 1;
            term(input, cur, out);
            translate(ch, out);
        }
    }
}

fn term(input: &[u8], cur: &mut usize, out: &mut Vec<String>) {
    factor(input, cur, out);

    loop {
        let ch = token(input, *cur);
        if ch == '$' || (ch != '*' && ch != '/') {
            break;
        } else {
            *cur += 1;
            factor(input, cur, out);
            translate(ch, out);
        }
    }
}

fn factor(input: &[u8], cur: &mut usize, out: &mut Vec<String>) {
    let ch = token(input, *cur);

    if ch.is_digit(10) {
        out.push(format!("push {}", ch));
    } else if ch == '(' {
        *cur += 1;
        expr(input, cur, out);

        let ch = token(input, *cur);
        if ch != ')' {
            panic!("Missing ')' at {}", *cur);
        }
    } else {
        panic!("Unexpected symbol '{}', at {}", ch, *cur);
    }

    *cur += 1;
}

fn translate(ch: char, out: &mut Vec<String>) {
    out.push(String::from("pop ebx"));
    out.push(String::from("pop eax"));
    out.push(format!("{} eax, ebx", to_oper(ch)));
    out.push(String::from("push eax"));
}

fn to_oper(ch: char) -> String {
    match ch {
        '+' => return String::from("add"),
        '-' => return String::from("sub"),
        '*' => return String::from("mul"),
        '/' => return String::from("div"),
        _ => panic!("Invalid operator '{}'", ch),
    }
}

pub fn main() {
    let mut out = vec![];
    let mut cur = 0;
    let exp = b"2/(7-3)";

    single_expr(exp, &mut cur, &mut out);
    assert_eq!(
        out,
        vec![
            "push 2",
            "push 7",
            "push 3",
            "pop ebx",
            "pop eax",
            "sub eax, ebx",
            "push eax",
            "pop ebx",
            "pop eax",
            "div eax, ebx",
            "push eax",
        ]
    );
}
```

## Solution 2

The second approach is using Rust's `macro_rules!`.
We simply define rules and leave the rest to Rust's
interpretation of these rules wich converts a given expression into
corresponding assembly code.
However, we have to make compromises on the input syntax
to make using standard repetitions of `macro_rules!` more tractable.
In thithe following example, we have to write `(2 * 3) - 5` instead of `2 * 3 - 5`.

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
    to_asm!((2 * 3) - 5);
    println!("-------------------");
    to_asm!(2 * (3 - 5));
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
