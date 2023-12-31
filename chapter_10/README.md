## Generic Types, Traits, and Lifetimes

Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties. We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

Functions can take parameters of some generic type, instead of a concrete type like i32 or String, in the same way a function takes parameters with unknown values to run the same code on multiple concrete values.
In fact, we’ve already used generics in Chapter 6 with Option<T>, Chapter 8 with Vec<T> and HashMap<K, V>, and Chapter 9 with Result<T, E>. In this chapter, you’ll explore how to define your own types, functions, and methods with generics!

First, we’ll review how to extract a function to reduce code duplication. We’ll then use the same technique to make a generic function from two functions that differ only in the types of their parameters. We’ll also explain how to use generic types in struct and enum definitions.

Then you’ll learn how to use traits to define behavior in a generic way. You can combine traits with generic types to constrain a generic type to accept only those types that have a particular behavior, as opposed to just any type.

Finally, we’ll discuss lifetimes: a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.

## Traits: Defining Shared Behavior

A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

## Validating References with Lifetimes

Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

"Permissions Are Returned At The End of a Reference's Lifetime" is that every reference in Rust has a lifetime, which is the scope for which that reference is valid.

Annotating lifetimes is not even a concept most other programming languages have, so this is going to feel unfamiliar.
Although we won’t cover lifetimes in their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax so you can get comfortable with the concept.

Lifetime Elision
The compiler has 3 rules for apply the lifetimes in Functions
1 for the input, and 2 for the output
Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

     1 - The first rule is that the compiler assigns a different lifetime parameter to each lifetime in each input type
     2 - If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
     3 - f there are multiple input lifetime parameters, but one of them is &self or
     &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
     see more in https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html#lifetime-elision
