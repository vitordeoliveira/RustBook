## Common Collections

### Module std::collections

Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. In this chapter, we’ll discuss three collections that are used very often in Rust programs:

### A vector
allows you to store a variable number of values next to each other.
### A string
is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
### A hash map
allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

## Storing UTF-8 Encoded Text with Strings
### What Is a String?
We’ll first define what we mean by the term string. Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. In Chapter 4, we talked about string slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types. Although this section is largely about String, both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

