# Functional Language Features: Iterators and Closures

<!--toc:start-->

- [Functional Language Features: Iterators and Closures](#functional-language-features-iterators-and-closures)
  - [Iterators](#iterators)
  <!--toc:end-->

Rust’s design has taken inspiration from many existing languages and techniques,
and one significant influence is functional programming. Programming in a
functional style often includes using functions as values by passing them in
arguments, returning them from other functions, assigning them to variables for
later execution, and so forth.

In this chapter, we won’t debate the issue of what functional programming is or
isn’t but will instead discuss some features of Rust that are similar to
features in many languages often referred to as functional.

More specifically, we’ll cover:

1. Closures, a function-like construct you can store in a variable
1. Iterators, a way of processing a series of elements
1. How to use closures and iterators to improve the I/O project in Chapter 12
1. The performance of closures and iterators (Spoiler alert: they’re faster than
   you might think!)

We’ve already covered some other Rust features, such as pattern matching and
enums, that are also influenced by the functional style. Because mastering
closures and iterators is an important part of writing idiomatic, fast Rust
code, we’ll devote this entire chapter to them.

Note: Functions can implement all three of the Fn traits too. If what we want to
do doesn’t require capturing a value from the environment, we can use the name
of a function rather than a closure where we need something that implements one
of the Fn traits. For example, on an Option\<Vec<\T\>\> value, we could call
unwrap_or_else(Vec::new) to get a new, empty vector if the value is None.

## Iterators

The iter method produces an iterator over immutable references. If we want to
create an iterator that takes ownership of v1 and returns owned values, we can
call into_iter instead of iter. Similarly, if we want to iterate over mutable
references, we can call iter_mut instead of iter.

The point is this: iterators, although a high-level abstraction, get compiled
down to roughly the same code as if you’d written the lower-level code yourself.
Iterators are one of Rust’s zero-cost abstractions, by which we mean using the
abstraction imposes no additional runtime overhead.
