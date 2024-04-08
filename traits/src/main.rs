// traits are like interfaces in other languages --> define abstract common behaviour for similar instances

use std::fmt::Display;

// we provided a method signature for the summarize trait --> any struct that implements the Summary trait must provide its own implementation of the summarize method
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // can specify default implementation that the type can override
    }
}
// a struct implementing a trait must provide implementations for all its methods

pub struct Article {
    pub headline: String,
    pub content: String,
    pub author: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub user: String,
    pub tweet_text: String
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.tweet_text, self.user)
    }
}

// One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.

/* passing traits as parameters */
pub fn notify(item: &impl Summary) { // we can only pass types that implement the summary trait
    println!("Breaking new! {}", item.summarize());
}

// trait bound syntax for the above syntactic sugar
// pub fn notify<T: Summary>(item: &T)
// pub fn notify<T: Summary>(item: &T)

// pub fn function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
/* Equivalent */
// pub fn function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug
// {}

// multiple traits
// pub fn notify(item: &(impl Summary + Display)) --> accepts type that implement both Summary and Display

/* Returning types that implement trait */
fn returns_summarizable() -> impl Summary {
    Tweet {
        tweet_text: "".to_string(),
        user: "".to_string()
    }
}

/* Using trait bounds to conditionally implement methods */
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("{} is larger", self.x);
        } else {
            println!("{} is larger", self.y);
        }
    }
}

// eg. to_string is an implementation for any type that implements Display trait
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
        
//     }
// }
// anything that implements the Display trait, implements the ToString trait.. 


fn main() {
    let p = Pair::new(2, 3);
    p.cmp_display();
    // can also be called as --> any implementation function that has &self is a method --> can be called from Type"::" or instance"."
    Pair::cmp_display(&p);

    println!("{}",
        Article {headline:"Big news".to_string(), 
                content: "Big news content".to_string(), 
                author: "Big news author".to_string() 
            }.summarize()
    );

    println!("{}",
        Tweet {
            user: "shakaal".to_string(),
            tweet_text: "chale jaoo!!".to_string()
        }.summarize()
    );
}
