// Traits define shared functionality, they are pretty cool.
// We can also use trait bounds to specify that the function can only be used when certain
// behaviour is guaranteed

use std::fmt::{Debug, Display};

// This defines a trait called Summary
// for a type to have the summary trait, they need to implement
// a summarize functions which takes it self and returns a string.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing summary for our custom type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// We can also have a default implementation of the function
pub trait Int {
    fn flatten(&self) -> usize {
        0
    }
}

pub struct Dummy {
    pub name: String,
}

// Giving dummy the default implementation of the trait
impl Int for Dummy {}

// This is why traits are kinda cool
// Instead of a concrete type, a function might ask for any type so long as they implement
// the summary trait for example.
pub fn notify(item: &impl Summary) {}

// Generic version
// They do the same thing as the one above
//pub fn notify<T: Summary>(item: &T) {}

// We can also ask for multiple traits
// This note asks for a type that implements both summary and int
pub fn note(item: &(impl Summary + Int)) {}
// Generic version (looks better in my opinion)
// pub fn note<T: Summary + Display>(item: &T){}
//
// We can also use the where clause to make it less cluterred
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// We can also return something that impls a trait
fn return_summarize() -> impl Summary {
    NewsArticle {
        author: "Me".to_string(),
        headline: "Good things are happening".to_string(),
        location: "Here".to_string(),
        content: "Good".to_string(),
    }
}

fn main() {
    let dummy = Dummy {
        name: "Adam".to_string(),
    };

    println!("The default implementation returns: {}", dummy.flatten());
}
