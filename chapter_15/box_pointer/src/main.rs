enum List {
    Cons(i32, Box<List>),
    Nil,
}

// use crate::List::{Cons, Nil};
use List::{Cons, Nil};

/// Box pointers allow us to have a 8bytes memory alocation pointing to another data,
/// That allow the compiler to know how much space allocate for the Enum, Struct, etc...
/// The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T>
/// values to be treated like references. When a Box<T> value goes out of scope, the heap data that
/// the box is pointing to is cleaned up as well because of the Drop trait implementation.
fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
