#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

/// There are situations in which it would be useful for a value to mutate itself in its methods
/// but appear immutable to other code. Code outside the value’s methods would not be able to
/// mutate the value. Using RefCell<T> is one way to get the ability to have interior mutability,
/// but RefCell<T> doesn’t get around the borrowing rules completely: the borrow checker in the
/// compiler allows this interior mutability, and the borrowing rules are checked at runtime
/// instead. If you violate the rules, you’ll get a panic! instead of a compiler error
fn main() {
    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Cell is typically used for simple types, as it requires copying or moving values.
    // More complex interior types typically use RefCell,
    // which tracks shared and exclusive references at runtime and panics if they are misused.
}
