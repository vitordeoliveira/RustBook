fn main() {
    // Specifying Placeholder Types in Trait Definitions with Associated Types

    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // Default Generic Type Parameters and Operator Overloading
    // You specify a default type when declaring a generic type with the
    // <PlaceholderType=ConcreteType> syntax.
    // An associated type cannot have a default, while a trait type parameter can have a default

    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // trait Add<Rhs = Self> {
    //     type Output;
    //
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Millimeters(u32);

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Meters {
        a: u32,
    }

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.a * 1000))
        }
    }

    assert_eq!(Millimeters(10) + Meters { a: 10 }, Millimeters(10010));
    // not implemented the oposite
    // assert_eq!(Meters(10) + Millimeters(10), Millimeters(10010));

    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    person.fly();

    Wizard::fly(&person);

    // associated functions that are not methods don’t have a self parameter
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Using Supertraits to Require One Trait’s Functionality Within Another Trait
    //
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point2 {
        x: i32,
        y: i32,
    }

    // after this line now Point2 need to implement Display because OutlinePrint force him to do
    // this
    impl OutlinePrint for Point2 {}

    impl fmt::Display for Point2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let point2 = Point2 { x: 0, y: 20 };
    point2.outline_print();

    // Using the Newtype Pattern to Implement External Traits on External Types
    // we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
