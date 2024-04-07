// traits are like interfaces in other languages --> define abstract common behaviour for similar instances

// we provided a method signature for the summarize trait --> any struct that implements the Summary trait must provide its own implementation of the summarize method
pub trait Summary {
    fn summarize(&self) -> String;
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

fn main() {
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
