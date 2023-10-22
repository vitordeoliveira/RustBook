fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// You can receive data from diff lifetimes, but just return de lifetime you want
fn _longest_two_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        "na"
    } else {
        y
    }
}

// if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice,
// we wouldn’t need to specify a lifetime on the y parameter
fn _longest_3<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}
// The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
// We’re also trying to return a reference to result from the function.
// There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference.

// fn _longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// fn longest_inv(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn main() {
    // Here, x has the lifetime 'b, which in this case is larger than 'a.
    // This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
    let x = 5; // ----------+-- 'b
               //           |
    let r = &x; // --+-- 'a  |
                //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
                          // ----------+

    // Generic Lifetimes in Functions
    // Lifetime Annotation Syntax
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Next exemple
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // Next exemple
    let string5 = String::from("long string is long");
    let result2;
    {
        let string2 = String::from("xyz");
        result2 = longest(string5.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result2);

    // Lifetime Annotations in Struct Definitions
    //
    // A struct that holds a reference, requiring a lifetime annotation
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
    // The compiler has 3 rules for apply the lifetimes in Functions
    // 1 for the input, and 2 for the output
    // Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
    //
    // 1 - The first rule is that the compiler assigns a different lifetime parameter to each lifetime in each input type
    // 2 - If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
    // 3 - f there are multiple input lifetime parameters, but one of them is &self or
    // &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
    // see more in https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html#lifetime-elision

    // Lifetime Annotations in Method Definitions
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // The Static Lifetime
    // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program.
    let s: &'static str = "I have a static lifetime.";

    // The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
