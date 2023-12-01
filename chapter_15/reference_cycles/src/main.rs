use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

/// The reference count of the Rc<List> instances in both a and b are 2 after we change the list in
/// a to point to b. At the end of main, Rust drops the variable b, which decreases the reference
/// count of the b Rc<List> instance from 2 to 1. The memory that Rc<List> has on the heap won’t be
/// dropped at this point, because its reference count is 1, not 0. Then Rust drops a, which
/// decreases the reference count of the a Rc<List> instance from 2 to 1 as well. This instance’s
/// memory can’t be dropped either, because the other Rc<List> instance still refers to it. The
/// memory allocated to the list will remain uncollected forever. To visualize this reference
/// cycle, we’ve created a diagram in Figure 15-4.
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // Let’s look at an example using graphs made up of parent nodes and child nodes to see when
    // non-ownership relationships are an appropriate way to prevent reference cycles.
    // Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
    // So far, we’ve demonstrated that calling Rc::clone increases the strong_count of an Rc<T>
    // instance, and an Rc<T> instance is only cleaned up if its strong_count is 0. You can also
    // create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and
    // passing a reference to the Rc<T>. Strong references are how you can share ownership of an
    // Rc<T> instance. Weak references don’t express an ownership relationship, and their count
    // doesn’t affect when an Rc<T> instance is cleaned up.
}
