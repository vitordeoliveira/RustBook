use std::ops::Deref;

/// Note: there’s one big difference between the MyBox<T> type we’re about to build and the real
/// Box<T>: our version will not store its data on the heap. We are focusing this example on Deref, so
/// where the data is actually stored is less important than the pointer-like behavior.
fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // NEXT
    // here we set y to be an instance of a Box<T> pointing to a copied value of x rather than a
    // reference pointing to the value of x.
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // NEXT
    // Defining Our Own Smart Pointer

    #[derive(Debug)]
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    let m = MyBox::new(String::from("Rust"));

    assert_eq!(5, x);

    hello(&(m.0)[..]);
    // won’t compile because Rust doesn’t know how to dereference MyBox.
    // assert_eq!(5, *y);

    // Treating a Type Like a Reference by Implementing the Deref Trait
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    assert_eq!(5, *y);

    // NEXT Implicit Deref Coercions with Functions and Methods

    // Here we’re calling the hello function with the argument &m, which is a reference to a
    // MyBox<String> value. Because we implemented the Deref trait on MyBox<T> in Listing 15-10,
    // Rust can turn &MyBox<String> into &String by calling deref. The standard library provides an
    // implementation of Deref on String that returns a string slice, and this is in the API
    // documentation for Deref. Rust calls deref again to turn the &String into &str, which matches
    // the hello function’s definition.
    // When the Deref trait is defined for the types involved, Rust will analyze the types and use
    // Deref::deref as many times as necessary to get a reference to match the parameter’s type. The
    // number of times that Deref::deref needs to be inserted is resolved at compile time, so there
    // is no runtime penalty for taking advantage of deref coercion!

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // If Rust didn’t implement deref coercion, we would have to write the code in Listing 15-13
    // instead of the code in Listing 15-12 to call hello with a value of type &MyBox<String>.
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
