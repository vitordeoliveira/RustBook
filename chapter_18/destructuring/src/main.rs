struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Destructuring Structs

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // we have a match expression that separates Point values into three cases: points that lie
    // directly on the x axis (which is true when y = 0), on the y axis (x = 0), or neither.
    let p = Point { x: 0, y: 7 };

    // The pattern still creates an x variable that we can use in the code for this arm.

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring Enums

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // For enum variants without any data, like Message::Quit, we can’t destructure the value any further.
    // So far, our examples have all been matching structs or enums one level deep, but matching can work on nested items too!
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // Destructuring Structs and Tuples
    //We can mix, match, and nest destructuring patterns in even more complex ways. The following
    //example shows a complicated destructure where we nest structs and tuples inside a tuple and
    //destructure all the primitive values out:
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
