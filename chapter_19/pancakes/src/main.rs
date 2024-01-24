use hello_macro_derive::route;
use hello_macro_derive::sql;
use hello_macro_derive::HelloMacro;
use macros::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[route(GET, "/")]
fn index() {
    // fn index is consumed, and generates the macros code
    // Comments are not send to the macro
}

fn main() {
    // Derive macro
    Pancakes::hello_macro();
    // Attribute-like macros
    attributeMacro();
    // Function-like macros
    sql!(pass whataver you want);
}
