mod private {
    use std::ops::Deref;

    pub struct CustomSmartPointer {
        data: String,
    }

    impl CustomSmartPointer {
        pub fn new(data: &str) -> CustomSmartPointer {
            CustomSmartPointer {
                data: String::from(data),
            }
        }

        fn _drop_method(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    impl Deref for CustomSmartPointer {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Called");
        }
    }
}

pub use private::CustomSmartPointer;

fn main() {
    let obj = CustomSmartPointer::new("5");

    println!("{}", *obj);
    // ... do some work with obj

    // obj.drop_method(); // This would be a compile error
}
