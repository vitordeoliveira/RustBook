fn main() {
    // Ignoring an Entire Value with _
    // Ignoring a function parameter can be especially useful in cases when, for example, you're
    // implementing a trait where you need a certain type signature but the function body in your
    // implementation doesn’t need one of the parameters.
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    // Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    // let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    // In all other cases (if either setting_value or new_setting_value are None) expressed by the _
    // pattern in the second arm, we want to allow new_setting_value to become setting_value.

    println!("setting is {:?}", setting_value);

    // NEXT

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _
    let _x = 5;

    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }

    // We’ll receive an error because the s value will still be moved into _s, which prevents us from
    // using s again. However, using the underscore by itself doesn’t ever bind to the value.

    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with ..
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 1 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // with tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // However, using .. must be unambiguous. If it is unclear which values are intended for matching
    // and which should be ignored, Rust will give us an error.
}
