#[macro_export]
macro_rules! myVec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
        // {
        //     let mut temp_vec = Vec::new();
        //     temp_vec.push(1);
        //     temp_vec.push(2);
        //     temp_vec.push(3);
        //     temp_vec
        // }
    };
}

fn main() {
    // Declarative macro
    // Note: The actual definition of the vec! macro in the standard library includes code to
    // preallocate the correct amount of memory up front. That code is an optimization that we don’t
    // include here to make the example simpler.
    let v: Vec<u32> = myVec![1, 2, 3];

    // Procedural Macros

    // use hello_macro_derive::HelloMacro;
    // use macros::HelloMacro;
    // struct Pancakes;
    //
    // impl HelloMacro for Pancakes {
    //     fn hello_macro() {
    //         println!("Hello, Macro! My name is Pancakes!");
    //     }
    // }
    //
    // Pancakes::hello_macro();
}
