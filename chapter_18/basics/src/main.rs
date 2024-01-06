fn main() {
    println!("Hello, world!");

    // here's the match expression from Listing 6-5 that matches on an Option<i32> value in the variable x:
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }

    // Conditional if let Expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // NEXT
    // for Loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // NEXT
    // let Statements
    // let PATTERN = EXPRESSION;
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    // Ignore pattern (we will explain more later)
    // To fix the error, we could ignore one or more of the values in the tuple using _ or ..
    let (x, y, ..) = (1, 2, 3);
    let (x, y, _) = (1, 2, 3);

    // NEXT
    // Function Parameters

    fn foo(x: i32) {
        // code goes here
    }

    // The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to
    // the pattern. Splits the values in a tuple as we pass it to a function.

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
}
