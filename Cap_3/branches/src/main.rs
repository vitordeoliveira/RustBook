fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Next exemple

    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Next exemple
    // You can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Next exemple
    // You can optionally specify a loop label on a loop that you can then use with break or
    //  continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    //  Loop labels must begin with a single quote. Here’s an example with two nested loops:

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Next Exemple
    println!();
    println!("--------");
    println!("Conditional Loops with while");
    println!();

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Next Exemple
    println!();
    println!("--------");
    println!("Looping Through a Collection with for");
    println!();

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!();
    println!("--------");
    println!();

    for number in 1..=4 {
        println!("{number}!");
    }
    for number in 1..4 {
        println!("{number}!");
    }
    for number in (1..3).rev() {
        println!("{number}!");
    }

    
    println!("LIFTOFF!!!");
}
