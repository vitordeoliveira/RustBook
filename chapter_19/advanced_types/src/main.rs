use std::fmt;

fn main() {
    // Using the Newtype Pattern for Type Safety and Abstraction
    // Newtypes can also hide internal implementation.
    // Including statically enforcing that values are never confused and indicating the units of a value.

    // NEXT
    // Creating Type Synonyms with Type Aliases
    // The main use case for type synonyms is to reduce repetition
    // we don’t get the type checking benefits that we get from the newtype pattern discussed
    // earlier. In other words, if we mix up Kilometers and i32 values somewhere, the compiler will
    // not give us an error.
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    // fn returns_long_type() -> Thunk {
    //     // --snip--
    // }

    // next exemple

    type Result<T> = std::result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }

    // The Never Type that Never Returns

    // Rust has a special type named ! that’s known in type theory lingo as the empty type because
    // it has no values. We prefer to call it the never type because it stands in the place of the
    // return type when a function will never return. Here is an example:
    fn bar() -> ! {
        // --snip--
        loop {
            panic!()
        }
    }
    // Functions that return never are called diverging functions

    // we discussed that match arms must all return the same type, so here we should not be able to
    // return "Hello"
    // As you might have guessed, a continue expression has the type !
    // Because ! can never have a value, Rust decides that the type of guess is u32.
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    trait TraitName<T> {
        fn unwrap(self) -> T;
    }

    // The never type is useful with the panic! macro as well.
    // Rust sees that val has the type T and panic! has the type !, so the result of the overall
    // match expression is T. This code works because panic! doesn’t produce a value; it ends the
    // program. In the None case, we won’t be returning a value from unwrap, so this code is valid.
    impl<T> TraitName<T> for Option<T> {
        fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }

    // One final expression that has the type ! is a loop:
    //     print!("forever ");
    //
    //     loop {
    //         print!("and ever ");
    //     }
    // Here, the loop never ends, so ! is the type of the expression. However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.

    // Dynamically Sized Types and the Sized Trait
    //
    //
}
