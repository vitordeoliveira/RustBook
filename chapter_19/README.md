# Advanced Features

<!--toc:start-->

- [Advanced Features](#advanced-features)
  - [Unsafe Superpowers](#unsafe-superpowers)
  - [Advanced Traits](#advanced-traits)
    - [Default Generic Type parameters](#default-generic-type-parameters)
      - [Operation Overloading](#operation-overloading)
      - [Using the Newtype Pattern to Implement External Traits on External Types](#using-the-newtype-pattern-to-implement-external-traits-on-external-types)
  - [Advanced Types](#advanced-types)
    - [Dynamically Sized Types and the Sized Trait](#dynamically-sized-types-and-the-sized-trait)
  - [Advanced Functions and Closures](#advanced-functions-and-closures)
    - [Function Pointers](#function-pointers)
    - [Returning Closures](#returning-closures)
  - [Macros](#macros)
    - [The Difference Between Macros and Functions](#the-difference-between-macros-and-functions)
    - [Declarative Macros with macrorules for General Metaprogramming](#declarative-macros-with-macrorules-for-general-metaprogramming)
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

#### Using the Newtype Pattern to Implement External Traits on External Types

In a nutshell:

we’re only allowed to implement a trait on a type if **EITHER** the trait
**OR** the
type are local to our crate. It’s possible to get around this restriction using
the newtype pattern, which involves creating a new type in a tuple struct.

## Advanced Types

The Rust type system has some features that we’ve so far mentioned but haven’t
yet discussed. We’ll start by discussing newtypes in general as we examine why
newtypes are useful as types. Then we’ll move on to type aliases, a feature
similar to newtypes but with slightly different semantics. We’ll also discuss
the ! type and dynamically sized types.

### Dynamically Sized Types and the Sized Trait

These types let us write code using values whose size we can know only at
runtime. We can’t know how long the string is until runtime, meaning we can’t
create a variable of type str, nor can we take an argument of type str.

Consider the following code, which does not work:

```rust
// does not compile
let s1: str = "Hello there!";
```

Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory.

If Rust allowed us to write this code, these two str values would need to take
up the same amount of space.

So what do we do? In this case, you already know the answer: we make the types
of s1 and s2 a &str rather than a str the slice data structure just stores the
starting position and the length of the slice.

&str is two values: the address of the str and its length. As such, we can know
the size of a &str value at compile time: it’s twice the length of a usize.
That is, we always know the size of a &str, no matter how long the string it
refers to is. In general, this is the way in which dynamically sized types are
used in Rust: they have an extra bit of metadata that stores the size of the
dynamic information.

> **The golden rule of dynamically
> sized types is that we must always put values of dynamically sized types behind
> a pointer of some kind.**

We can combine str with all kinds of pointers: for example, Box\<str\> or Rc\<str\>
In fact, you’ve seen this before but with a different dynamically sized type: traits

we mentioned that to use traits as trait objects, we must put them behind a
pointer, such as &dyn Trait or Box\<dyn Trait\> (Rc\<dyn Trait\> would work too).

```rust
fn generic<T>(t: T) {
    // --snip--
}
// equal to
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will work only on types that have a known size at
compile time. However, you can use the following special syntax to relax this
restriction:

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

A trait bound on ?Sized means “T may or may not be Sized” and this notation
overrides the default that generic types must have a known size at compile
time. The ?Trait syntax with this meaning is only available for Sized, not any
other traits.

Also note that we switched the type of the t parameter from T to &T. Because
the type might not be Sized, we need to use it behind some kind of pointer. In
this case, we’ve chosen a reference.

## Advanced Functions and Closures

This section explores some advanced features related to functions and closures,
including function pointers and returning closures.

### Function Pointers

We’ve talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a
function you’ve already defined rather than defining a new closure. Functions
coerce to the type fn (with a lowercase f), not to be confused with the Fn
closure trait. The fn type is called a function pointer. Passing functions with
function pointers will allow you to use functions as arguments to other
functions.

That said, one example of where you would want to only accept fn and not
closures is when interfacing with external code that doesn’t have closures: C
functionos can accept functions as arguments, but C doesn’t have closures.

### Returning Closures

Here we create Status::Value instances using each u32 value in the range that
map is called on by using the initializer function of Status::Value. Some
people prefer this style, and some people prefer to use closures. They compile
to the same code, so use whichever style is clearer to you.

The following code tries to return a closure directly, but it won’t compile:

```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
|x| x + 1
}
```

## Macros

We’ve used macros like println! throughout this book, but we haven’t fully
explored what a macro is and how it works. The term macro refers to a family of
features in Rust: declarative macros with macro_rules! and three kinds of
procedural macros:

- Custom #[derive] macros that specify code added with the derive attribute
  used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens
  specified as their argument

### The Difference Between Macros and Functions

Fundamentally, macros are a way of writing code that writes other code, which
is known as metaprogramming.

A function signature must declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters: we can call println!("hello") with one argument or println!("hello
{}", name) with two arguments. Also, macros are expanded before the compiler
interprets the meaning of the code, so a macro can, for example, implement a
trait on a given type. A function can’t, because it gets called at runtime and
a trait needs to be implemented at compile time.

Another important difference between macros and functions is that you must
define macros or bring them into scope before you call them in a file, as
opposed to functions you can define anywhere and call anywhere.

### Declarative Macros with macrorules for General Metaprogramming
