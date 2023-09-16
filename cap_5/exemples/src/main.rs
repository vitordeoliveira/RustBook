struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // struct update syntax

    // without
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // with
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // 5.2
    // An Example Program Using Structs

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_0(width1, height1)
    );

    fn area_0(width: u32, height: u32) -> u32 {
        width * height
    }

    // with tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_1(rect1)
    );

    fn area_1(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    // with structs
    struct Rectangle_0 {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle_0 {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(&rect1)
    );

    fn area_2(rectangle: &Rectangle_0) -> u32 {
        rectangle.width * rectangle.height
    }

    // Adding Useful Functionality with Derived Traits
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:?}", rect1);

    #[derive(Debug)]
    struct Rectangle_2 {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle_2 {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // Method Syntax
    #[derive(Debug)]
    struct Rectangle3 {
        width: u32,
        height: u32,
    }

    impl Rectangle3 {
        // The &self is actually short for self: &Self.
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle3 {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Note that we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle that is also named width:
    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // next

    #[derive(Debug)]
    struct Rectangle5 {
        width: u32,
        height: u32,
    }

    impl Rectangle5 {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle5) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle5 {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle5 {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle5 {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // We can define associated functions as functions that don’t have self as their first parameter (and thus are not methods)
    // because they don’t need an instance of the type to work with.
    // We’ve already used one function like this: the String::from function that’s defined on the String type.
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    // These are often called new, but new isn’t a special name and isn’t built into the language.

    impl Rectangle5 {
        // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword,
        // which in this case is Rectangle.
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let _sq = Rectangle5::square(3);

    // NExt exemple

    #[derive(Debug)]
    struct Rectangle6 {
        width: u32,
        height: u32,
    }

    impl Rectangle6 {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn set_width(&mut self, width: u32) {
            self.width = width;
        }

        fn max(self, other: Rectangle) -> Rectangle {
            Rectangle { 
                width: self.width.max(other.width),
                height: self.height.max(other.height),
            }
        }

        // fn set_to_max(&mut self, other: Rectangle) {
        //     *self = self.max(other);
        // }
    }

    let mut r = Rectangle6 {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle6::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle6::set_width(&mut r, 2);

    // next

    let r = &mut Box::new(Rectangle6 {
        width: 1,
        height: 2,
    });
    let area1 = r.area();
    let area2 = Rectangle6::area(&**r);
    assert_eq!(area1, area2);

    // Next
    #[derive(Debug)]
    #[derive(Copy, Clone)]
    struct Rectangle7 {
        width: u32,
        height: u32,
    }
    impl Rectangle7 {    
        fn max(self, other: Self) -> Self {
          let w = self.width.max(other.width);
          let h = self.height.max(other.height);
          Rectangle7 { 
            width: w,
            height: h
          }
        }
          fn set_to_max(&mut self, other: Rectangle7) {
              let max = self.max(other);
              *self = max;
          }
      }

    let mut rect = Rectangle7 { width: 0, height: 1 };
    let other_rect = Rectangle7 { width: 1, height: 0 };
    rect.set_to_max(other_rect);
}
