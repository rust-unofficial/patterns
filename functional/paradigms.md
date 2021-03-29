# Programming paradigms

One of the biggest hurdles to understanding functional programs when coming
from an imperative background is the shift in thinking. Imperative programs
describe __how__ to do something, whereas declarative programs describe
__what__ to do. Let's sum the numbers from 1 to 10 to show this.

## Imperative

```rust
let mut sum = 0;
for i in 1..11 {
    sum += i;
}
println!("{}", sum);
```

With imperative programs, we have to play compiler to see what is happening.
Here, we start with a `sum` of `0`.
Next, we iterate through the range from 1 to 10.
Each time through the loop, we add the corresponding value in the range.
Then we print it out.

| `i` | `sum` |
|:---:|:-----:|
|   1 |     1 |
|   2 |     3 |
|   3 |     6 |
|   4 |    10 |
|   5 |    15 |
|   6 |    21 |
|   7 |    28 |
|   8 |    36 |
|   9 |    45 |
|  10 |    55 |

This is how most of us start out programming. We learn that a program is a set
of steps.

## Declarative

```rust
println!("{}", (1..11).fold(0, |a, b| a + b));
```

Whoa! This is really different! What's going on here?
Remember that with declarative programs we are describing __what__ to do,
rather than __how__ to do it. `fold` is a function that [composes](https://en.wikipedia.org/wiki/Function_composition)
functions. The name is a convention from Haskell.

Here, we are composing functions of addition (this closure: `|a, b| a + b`)
with a range from 1 to 10. The `0` is the starting point, so `a` is `0` at
first. `b` is the first element of the range, `1`. `0 + 1 = 1` is the result.
So now we `fold` again, with `a = 1`, `b = 2` and so `1 + 2 = 3` is the next
result. This process continues until we get to the last element in the range,
`10`.

| `a` | `b` | result |
|:---:|:---:|:------:|
|   0 |   1 |      1 |
|   1 |   2 |      3 |
|   3 |   3 |      6 |
|   6 |   4 |     10 |
|  10 |   5 |     15 |
|  15 |   6 |     21 |
|  21 |   7 |     28 |
|  28 |   8 |     36 |
|  36 |   9 |     45 |
|  45 |  10 |     55 |
