# Object-Oriented Programming Features of Rust

<!--toc:start-->

- [Object-Oriented Programming Features of Rust](#object-oriented-programming-features-of-rust)
<!--toc:end-->

In this chapter, we’ll explore certain characteristics that are commonly
considered object-oriented and how those characteristics translate to idiomatic
Rust. We’ll then show you how to implement an object-oriented design pattern in
Rust and discuss the trade-offs of doing so versus implementing a solution
using some of Rust’s strengths instead.

This concept—of being concerned only with the messages a value responds to
rather than the value’s concrete type—is similar to the concept of duck typing
in dynamically typed languages: if it walks like a duck and quacks like a duck,
then it must be a duck! In the implementation of run on Screen in Listing 17-5,
run doesn’t need to know what the concrete type of each component is. It
doesn’t check whether a component is an instance of a Button or a SelectBox, it
just calls the draw method on the component. By specifying Box\<dyn Draw\> as the
type of the values in the components vector, we’ve defined Screen to need
values that we can call the draw method on.

Recall in the “Performance of Code Using Generics” section in Chapter 10 our
discussion on the monomorphization process performed by the compiler when we
use trait bounds on generics: the compiler generates nongeneric implementations
of functions and methods for each concrete type that we use in place of a
generic type parameter. The code that results from monomorphization is doing
static dispatch, which is when the compiler knows what method you’re calling at
compile time. This is opposed to dynamic dispatch, which is when the compiler
can’t tell at compile time which method you’re calling. In dynamic dispatch
cases, the compiler emits code that at runtime will figure out which method to
call.
