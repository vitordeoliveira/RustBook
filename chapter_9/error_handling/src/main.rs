use std::{fs::File, io::{ErrorKind, self, Read}};

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
}


fn read_username_from_file() -> Result<String, io::Error>{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
