// let’s say we have multiple structs that hold various kinds and amounts of text:
// a NewsArticle struct that holds a news story filed in a particular location and a Tweet that can have
// at most 280 characters along with metadata that indicates whether it was a new tweet, a retweet, or a reply to another tweet.
// We want to make a media aggregator library crate named aggregator that can display summaries of data
// that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type,
// and we’ll request that summary by calling a summarize method on an instance.

use std::fmt::{Debug, Display};

pub fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

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

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
// The "impl Trait" syntax is actually syntactic sugar for a longer form, called trait bound
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
// Say we wanted notify to use display formatting on item as well as the summarize method:
// we specify in the notify definition that item must implement both Display and Summary.
pub fn notify_multiple_trait_bounds(item: &(impl Summary + Display)) {}
pub fn notify_multiple_trait_bounds2<T: Summary + Display>(item: &T) {}

// Clearer trait bounds with where clauses
// Using too many trait bounds has its downsides. Each generic has its own trait bounds,
// so functions with multiple generic type parameters can contain lots of trait bound information
// between the function’s name and its parameter list, making the function signature hard to read.
// For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature.
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
            U: Clone + Debug {
    2
}

// Return types that implement traits
// The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// implements cmp_display() if the inner type T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

