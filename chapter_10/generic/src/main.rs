fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// TODO: Missing PartialOrd Trait

// There are no core methods in Rust. Without restrictions,
// a generic type T has no capabilities:
// it cannot be printed, cloned, or mutated (although it can be dropped).

// fn largest_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

// We can also define structs to use a generic type parameter in one or more fields using the <> syntax
struct Point<T> {
    x: T,
    y: T,
}

// If you want to allow x and y to be different types you should do that:
struct _Point2<T, U> {
    x: T,
    y: U,
}

// In Method Definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    //You cant impl a method like this for generic type
    //Rust does not have inherentance types
    // fn distance_from_origin(&self) -> f32 {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // }
}

// We can also specify constraints on generic types when defining methods on the type.
// We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let re: ult = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // NEXT EXEMPLE (POINT)
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 1.0, y: 2.0 };

    // NEXT exemple (POINT 2)
    let p3 = _Point2 { x: 10, y: 1.95 };
    println!("p1.x = {}", p1.x);
    println!("p1.x = {}", p1.x());

    println!("p2.dist = {}", p2.distance_from_origin());

    // Next exemple (POINT 3)
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
