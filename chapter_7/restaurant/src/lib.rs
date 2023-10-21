pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod back_of_house;
mod front_of_house;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {:?} toast please", meal);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// use is binded to crate mod scope
use crate::front_of_house::hosting;
// use front_of_house::hosting;
mod customer {
    // use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        crate::hosting::add_to_waitlist();
        super::hosting::add_to_waitlist();
    }
}

use std::fmt::Result;
use std::io::Result as IoResult;

// NEXT

mod front_of_house_2;

pub use crate::front_of_house::hosting as hosting_2;

pub fn eat_at_restaurant_2() {
    hosting_2::add_to_waitlist();
}

// Before this change, external code would have to call the add_to_waitlist
// function by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// Now that this pub use has re-exported the hosting module from the root module, external code can now use the path
// restaurant::hosting::add_to_waitlist() instead.
