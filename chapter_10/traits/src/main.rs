use std::fmt::{Debug, Display};

use traits::aggregator::{NewsArticle, Summary2, Summary3, Tweet};
trait SummaryTest {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
impl SummaryTest for i32 {}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Traits as Parameters
    // We can call notify and pass in any instance of NewsArticle or Tweet.
    // Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.
    pub fn _notify(item: &impl Summary2) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound Syntax
    // This longer form is equivalent to the example in the previous section but is more verbose.
    // We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.
    pub fn _notify2<T: Summary2>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Using impl Trait is appropriate if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary)
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {

    // NEXT
    // Specifying Multiple Trait Bounds with the + Syntax
    // Say we wanted notify to use display formatting as well as summarize on item:
    // we specify in the notify definition that item must implement both Display and Summary.
    pub fn _notify3(_item: &(impl Summary2 + Display)) {}
    // trait bounds
    pub fn _notify4<T: Summary2 + Display>(_item: &T) {}

    // Will work
    _notify(&article);
    // Will not work because Display trait is not impl
    // _notify4(&article);

    // Clearer Trait Bounds with where Clauses
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    fn _some_function<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        0
    }

    // Returning Types that Implement Traits
    // You can only use impl Trait if you’re returning a single type.
    // For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work
    // fn _returns_summarizable(v: u32) -> impl Summary3 {
    //     if (v == 0) {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //             reply: false,
    //             retweet: false,
    //         }
    //     } else {
    //         NewsArticle {
    //             headline: String::from("Penguins win the Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //             ),
    //         }
    //     }
    // }

    // Using Trait Bounds to Conditionally Implement Methods
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd + SummaryTest> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // impl SummaryTest for Pair<i32> {}
    impl<T> SummaryTest for Pair<T> {}

    let a = Pair::new(10, 20);
    a.cmp_display();
    println!("{}", a.summarize());

    // We can also conditionally implement a trait for any type that implements another trait.
    // Implementations of a trait on any type that satisfies the trait bounds are called
    // blanket implementations and are extensively used in the Rust standard library.
    // impl<T: Display> ToString for T {
    //     // --snip--
    // }
    let _s = 3.to_string();
}
