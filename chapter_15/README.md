# Smart Pointers

Smart pointers are usually implemented using structs. Unlike an ordinary
struct, smart pointers implement the `Deref` and `Drop` traits.

The `Deref` trait allows an instance of the smart pointer struct to behave like
a reference so you can write your code to work with either references or smart
pointers.

The`Drop` trait allows you to customize the code that’s run when an instance of
the smart pointer goes out of scope. In this chapter, we’ll discuss both traits
and demonstrate why they’re important to smart pointers.

Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every existing smart pointer. Many
libraries have their own smart pointers, and you can even write your own. We’ll
cover the most common smart pointers in the standard library:

- Box\<T\> for allocating values on the heap
- Rc\<T\>, a reference counting type that enables multiple ownership
- Ref\<T\> and RefMut\<T\>, accessed through RefCell\<T\>, a type that enforces
  the borrowing rules at runtime instead of compile time

In addition, we’ll cover the interior mutability pattern where an immutable
type exposes an API for mutating an interior value. We’ll also discuss
reference cycles: how they can leak memory and how to prevent them.

## Box

### You’ll use them most often in these situations

- When you have a type whose size can’t be known at compile time and you want
  to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but
  ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that
  implements a particular trait rather than being of a specific type

## Deref

### Trait to manage the dereference with \*var

```rust
impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

// OR

impl Deref for CustomSmartPointer {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
```

## Drop

### Trait to manage the destructor

```rust
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Called");
    }
}
```

## RC\<T\>

In the majority of cases, ownership is clear: you know exactly which variable
owns a given value. However, there are cases when a single value might have
multiple owners. For example, in graph data structures, multiple edges might
point to the same node, and that node is conceptually owned by all of the edges
that point to it. A node shouldn’t be cleaned up unless it doesn’t have any
edges pointing to it and so has no owners.
