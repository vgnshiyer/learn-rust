use std::{cmp::Ordering, io};
use rand::{thread_rng, Rng}; // rand is a crate and thread_rng is a function in the crate

fn main() {
    println!("Guess the secret number!");

    #[allow(unused_variables)] // there are called compiler attributes that apply to methods, functions & variables
    let secret: u32 = thread_rng().gen_range(1..=100);

    loop {
        println!("Make a guess!");
        let mut guess = String::new();

        // :: --> call a function in a module
        // . --> call a method in a class(struct)
        io::stdin()
            .read_line(&mut guess)
            .expect("Enter a valid number");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Parse Error: {e}");
                break;
            },
        };
    
        println!("Your guess {guess}");
    
        // match is a powerful version of if/else (handles more complex code in a concise manner than if/else)
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You are right!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }

}
