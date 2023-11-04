pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Testing Equality with the assert_eq! and assert_ne! Macros
/// this is such a common test that the standard library provides a pair of macros—assert_eq! and assert_ne!—to perform this test more conveniently.
/// These macros compare two arguments for equality or inequality, respectively.
///  All primitive types and most of the standard library types implement these traits.
///  For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types.
///  You’ll also need to implement Debug to print the values when the assertion fails.
///  Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5,
///  this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.
///  See Appendix C, “Derivable Traits,” for more details about these and other derivable traits.
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // Note that in some languages and test frameworks, the parameters to equality assertion functions are called expected and actual,
        // and the order in which we specify the arguments matters.
        // However, in Rust, they’re called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter.
        assert_eq!(4, add_two(2));
        // assert_eq!(4, add_two(3), "Number should be 4 not {}", add_two(3));
        assert_ne!(5, add_two(2));
    }
    // Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // We can also write tests that use Result<T, E>! Here’s the test from Listing 11-1, rewritten to use Result<T, E> and return an Err instead of panicking:
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // You can’t use the #[should_panic] annotation on tests that use Result<T, E>.
    // To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value.
    // Instead, use assert!(value.is_err()).
}

// Testing Private Functions
// There’s debate within the testing community about whether or not private functions should be tested directly,
// and other languages make it difficult or impossible to test private functions.
// Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions.
// Consider the code in Listing 11-12 with the private function internal_adder.
#[cfg(test)]
mod tests_private {
    use super::*;
    pub fn add_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }

    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn internal() {
            assert_eq!(4, internal_adder(2, 2));
        }
    }
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
