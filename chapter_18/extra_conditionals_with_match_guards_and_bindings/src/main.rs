fn main() {
    // Match guard
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // we could use match guards to solve our pattern-shadowing problem.
    let x = Some(10);
    let y = 10;

    // This y is the outer y rather than a new shadowed y, and we can look for a value that has the
    // same value as the outer y by comparing n to y.

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // You can also use the or operator | in a match guard to specify multiple patterns;
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // The precedence of a match guard in relation to a pattern behaves like this:
    // (4 | 5 | 6) if y => ...
    // rather than this:
    // 4 | 5 | (6 if y) => ...
    //
    //
    //
    //
    // NEXT
    // @ Bindings

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
