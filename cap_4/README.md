## Understanding Ownership

Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

Pointer Safety Principle: data should never be aliased and mutated at the same time.

Aliasing is accessing the same data through different variables
Mutation is changing the data itself



1. All variables can read, own, and (optionally) write their data.
2. Creating a reference will transfer permissions from the borrowed path to the reference.
3. Permissions are returned once the reference's lifetime has ended.
4. Data must outlive all references that point to it.