use rand::Rng;
use std::collections::HashMap;
use restaurant::eat_at_restaurant;

pub mod point {
    #[derive(Debug)]
    pub struct Point(pub i32, pub i32);
    impl Point {
        pub fn origin() -> Self {
            Point(0, 0)
        }
    }
}
fn main() {
    eat_at_restaurant();
    println!("I'm growing!");

    let mut p = point::Point::origin();
    p.0 += 1;
    println!("{p:?}");

    // NEXT

    let secret_number = rand::thread_rng().gen_range(1..=100);
}
