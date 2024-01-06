fn main() {
    // Matching Literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    // The pattern in the second match arm introduces a new variable named y that will match any
    // value inside a Some value. Because we’re in a new scope inside the match expression, this is
    // a new y variable, not the y we declared at the beginning with the value 10.
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // To create a match expression that compares the values of the outer x and y, rather than
    // introducing a shadowed variable, we would need to use a match guard conditional instead.
    // We’ll talk about match guards later in the “Extra Conditionals with Match Guards” section.

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // ranges are only allowed with numeric or char values.
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
