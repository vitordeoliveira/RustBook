use std::{
    fs::File,
    io::{self, ErrorKind, Read}, net::IpAddr,
};

use error_handling::{chapter_2};
fn main() {
    println!("Unrecoverable Errors with panic!");
    // panic!("crash and burn");
    println!("Recoverable Errors with Result");
    let greeting_file_result = File::open("hello.txt");
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem opening file: {:?}", error),
    // };

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    // With Closures
    let _greeting_file_1 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro for us.
    // Here is an example of unwrap in action:

    let _greeting_file_2 = File::open("hello.txt").unwrap();

    // Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.
    // The syntax of expect looks like this:

    let _greeting_file_3 =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    println!("{}", _last_char_of_first_line("hello").unwrap());

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    
    // creating_custom_types_for_validation();
    chapter_2::creating_custom_types_for_validation();
}

fn _read_username_from_file_0() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.
fn _read_username_from_file_1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler.
// We could even shorten this code further by chaining method calls immediately after the ?
fn _read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// use std::fs::File;
// fn main() {
//     let greeting_file = File::open("hello.txt")?;
// }
// Listing 9-10: Attempting to use the ? in the main function that returns () won’t compile
// This code opens a file, which might fail. The ? operator follows the Result value returned by File::open,
// but this main function has the return type of (), not Result. When we compile this code, we get the following error message:
//
//? can be used with Option<T> values as well. As with using ? on Result, you can only use ? on Option in a function that returns an Option.
//The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>:
//if the value is None, the None will be returned early from the function at that point.
//If the value is Some, the value inside the Some is the resulting value of the expression and the function continues.
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

use std::error::Error;


fn _main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
