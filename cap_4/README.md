## Understanding Ownership

Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

Pointer Safety Principle: data should never be aliased and mutated at the same time.

Aliasing is accessing the same data through different variables
Mutation is changing the data itself

1. All variables can read, own, and (optionally) write their data.
2. Creating a reference will transfer permissions from the borrowed path to the reference.
3. Permissions are returned once the reference's lifetime has ended.
4. Data must outlive all references that point to it.

if a value does not own heap data, then it can be copied without a move. For example:

An i32 does not own heap data, so it can be copied without a move.

A String does own heap data, so it can not be copied without a move.

An &String does not own heap data, so it can be copied without a move.

Summary on Slices
Slices are a special kind of reference that refer to sub-ranges of a sequence, like a string or a vector. At runtime, a slice is represented as a "fat pointer" which contains a pointer to the beginning of the range and a length of the range. One advantage of slices over index-based ranges is that the slice cannot be invalidated while it's being used.

    Slices are a special kind of reference that refer to a contiguous sequence of data in memory.
    This diagram illustrates how a slice refers to a subsequence of characters in a string:

```
fn main() {
    let s = String::from("abcdefg");
    let s_slice = &s[2..5];
}
```



### This diagram illustrates how each concept looks at runtime:
    let mut a_num = 0;
    inner(&mut a_num);

    fn inner(x: &mut i32) {
        let another_num = 1;
        let a_stack_ref = &another_num;

        let a_box = Box::new(2);
        let a_box_stack_ref = &a_box;
        let a_box_heap_ref = &*a_box;

        *x += 5;
    }

* **Why does a_box_stack_ref point to the stack, while a_box_heap_ref point to the heap?**

* **Why is the value 2 no longer on the heap at L2?**

* **Why does a_num have the value 5 at L2?**

If you want to review boxes, re-read Chapter 4.1. If you want to review references, re-read Chapter 4.2. If you want to see case studies involving boxes and references, re-read Chapter 4.3.
