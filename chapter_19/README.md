# Advanced Features

<!--toc:start-->

- [Advanced Features](#advanced-features)
  - [Unsafe Superpowers](#unsafe-superpowers)
  - [Advanced Traits](#advanced-traits) - [Default Generic Type
  parameters](#default-generic-type-parameters) - [Operation
  Overloading](#operation-overloading)
  <!--toc:end-->

By now, you’ve learned the most commonly used parts of the Rust programming
language. Before we do one more project in Chapter 20, we’ll look at a few
aspects of the language you might run into every once in a while, but may not
use every day. You can use this chapter as a reference for when you encounter
any unknowns. The features covered here are useful in very specific situations.
Although you might not reach for them often, we want to make sure you have a
grasp of all the features Rust has to offer.

In this chapter, we’ll cover:

- Unsafe Rust: how to opt out of some of Rust’s guarantees and take
  responsibility for manually upholding those guarantees
- Advanced traits: associated types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
- Advanced types: more about the newtype pattern, type aliases, the never type,
  and dynamically sized types
- Advanced functions and closures: function pointers and returning closures
- Macros: ways to define code that defines more code at compile time

It’s a panoply of Rust features with something for everyone! Let’s dive in!

## Unsafe Superpowers

To switch to unsafe Rust, use the unsafe keyword and then start a new block
that holds the unsafe code. You can take five actions in unsafe Rust that you
can’t in safe Rust, which we call unsafe superpowers. Those superpowers include
the ability to:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe traits
- Access fields of unions

## Advanced Traits

### Default Generic Type parameters

\<PlaceholderType=ConcreteType\> syntax.

```rust
pub trait Iterator<MEAOW = String> {
    type BLA;
    fn next(&mut self, cat: MEAOW) -> Option<Self::BLA>;
}

impl Iterator<u32> for Counter {
    type BLA = i32;
    fn next(&mut self, cat: u32) -> Option<Self::BLA> {
        println!("{cat}");
        Some(10)
    }
}
// With associated types, we don’t need to annotate types because we can’t
// implement a trait on a type multiple times.
```

#### Operation Overloading

We have exemples in main.rs
