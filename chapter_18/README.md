# Patterns and Matching

<!--toc:start-->

- [Patterns and Matching](#patterns-and-matching)
  - [match Arms](#match-arms)
  <!--toc:end-->

Patterns are a special syntax in Rust for matching against the structure of
types, both complex and simple. Using patterns in conjunction with match
expressions and other constructs gives you more control over a program’s
control flow. A pattern consists of some combination of the following:

Literals
Destructured arrays, enums, structs, or tuples
Variables
Wildcards
Placeholders

This chapter is a reference on all things related to patterns. We’ll cover the
valid places to use patterns, the difference between refutable and irrefutable
patterns, and the different kinds of pattern syntax that you might see. By the
end of the chapter, you’ll know how to use patterns to express many concepts in
a clear way.

## Match Arms

As discussed in Chapter 6, we use patterns in the arms of match expressions.
Formally, match expressions are defined as the keyword match, a value to match
on, and one or more match arms that consist of a pattern and an expression to
run if the value matches that arm’s pattern, like this:

```rust
match VALUE {
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
}
```

For example, here's the match expression from Listing 6-5 that matches on an
Option\<i32\> value in the variable x:

```rust
match x {
  None => None,
  Some(i) => Some(i + 1),
}
```

The patterns in this match expression are the None and Some(i) on the left of
each arrow.

One requirement for match expressions is that they need to be exhaustive in the
sense that all possibilities for the value in the match expression must be
accounted for. One way to ensure you’ve covered every possibility is to have a
catchall pattern for the last arm: for example, a variable name matching any
value can never fail and thus covers every remaining case.

The particular pattern \_ will match anything, but it never binds to a variable,
so it’s often used in the last match arm. The \_ pattern can be useful when you
want to ignore any value not specified, for example.

## Conditional if let expressions

Is possible to use if let EVEN to Option or Result types

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
`
```

The downside of using if let expressions is that the compiler doesn’t check for
exhaustiveness, whereas with match expressions it does. If we omitted the last
else block and therefore missed handling some cases, the compiler would not
alert us to the possible logic bug.

In some places, the patterns must be irrefutable; in other circumstances, they
can be refutable.

## Refutable Patterns

> In the expression if let Some(x) = a_value, then Some(x) is refutable. If the
> value in the a_value variable is None rather than Some, the Some(x) pattern
> will not match.
> In the expression if let &\[x, ..\] = a_slice, then &\[x, ..\] is refutable. If
> the value in the a_slice variable has zero elements, the &\[x, ..\] pattern
> will not match.
