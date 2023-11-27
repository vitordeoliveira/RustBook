/// There are situations in which it would be useful for a value to mutate itself in its methods
/// but appear immutable to other code. Code outside the value’s methods would not be able to
/// mutate the value. Using RefCell<T> is one way to get the ability to have interior mutability,
/// but RefCell<T> doesn’t get around the borrowing rules completely: the borrow checker in the
/// compiler allows this interior mutability, and the borrowing rules are checked at runtime
/// instead. If you violate the rules, you’ll get a panic! instead of a compiler error

fn main() {
    println!("Hello, world!");
}
