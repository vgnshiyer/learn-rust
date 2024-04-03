// match is rust's pattern matching keyword, that allows you to define the behavior on certain pattern matches. The first pattern that matches will be implemented.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        // the match expression has keyword followed by expression
        // the result of the expression is returned to the left side of the match expression
        match self {
            Coin::Penny => 1, // in this case the matching keyword returns the u8 to the function caller
            Coin::Nickel => 5, // these are called as matching arms
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }

        // match is exhaustive.. It will give compilation error until all possible keywords are implemented.
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("{}", coin.value_in_cents());
}
