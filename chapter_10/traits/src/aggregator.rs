pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary2 for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type
pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary2 for NewsArticle {}

// Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.
pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// he forces U to implement summarize_author
impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
