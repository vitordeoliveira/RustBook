// const TEST: u32 = 10;

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x + TEST is: {}", x + TEST);
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// Data Types
// fn main(){
//     let _guess: u32 = "42".parse().expect("Not a number!");

//     // let tup0:(i32, f64, u8)= (500, 6.4, 1);
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
//     println!("The value of z is: {z}");

//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let _five_hundred = x.0;
//     let _six_point_four = x.1;
//     let _one = x.2;

//     let a = [1, 2, 3, 4, 5];

//     let _b: [i32; 5] = [1, 2, 3, 4, 5];

//     // [3, 3, 3, 3, 3]
//     let _c = [3; 5];

//     let _first = a[0];
//     let _second = a[1];
// }

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// fn main() {
//     //  is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement.
//     //  Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far.
//     //  Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement,
//     //  and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }


// Note it use -> and not =>
fn five() -> i32 {
    5
}

/// ## Documentation test
/// hello mom :D
/// 
/// RUN cargo doc --open
///
fn plus_one(x: i32) -> i32 {
    // Expression
    x + 1
    // Statement
    // x + 1;
}

fn _unit_exemple() -> () {
    // Expression
    // x + 1
    // Statement
    // x + 1;
}
fn main() {
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}