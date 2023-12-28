use oop::{Button, Draw, Screen};

fn main() {
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
        }
    }

    // NEXT
    //
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();

    // NEXT
    // One downside to using trait objects is how they interact with type inference.
    // For example, consider type inference for Vec<T>. When T is not a trait object, Rust just
    // needs to know the type of a single element in the vector to infer T
    // or LINE 48 or LINE 57 to compiler knows the type object
    // Vec<Box<dyn Draw>>
    // or
    // as Box<dyn Draw>
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }) as Box<dyn Draw>,
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];
    let screen = Screen { components };
    screen.run();
}
