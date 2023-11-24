struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/// Variables are dropped in the reverse order of their creation, so d was dropped before c.
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    //Occasionally, however, you might want to clean up a value early. One example is when using
    //smart pointers that manage locks: you might want to force the drop method that releases the
    //lock so that other code in the same scope can acquire the lock.

    //{
    //     c;
    // }
    // drop(c);
    //
    // print!("{}", c.data);
    println!("CustomSmartPointers created.");
}
