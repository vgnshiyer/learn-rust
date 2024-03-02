use std::env; // env module provides several useful functions
use std::str::FromStr; // bring trait (collection of methods) FromStr into scope

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // panic if n or m is 0
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}

fn main() {
    let mut numbers = Vec::new(); // vector --> growable datatype

    for arg in env::args().skip(1) {
        // the u64::from_str() method does not directly return a u64, but a Result
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    // Result has two values (Ok and Err) -> the expect method prints the message on Err and simply returns the parsed value on success (Ok)

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ..."); // eprintln! macro writes an error to std output
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        // & --> borrows a reference of the elements in numbers (ownership of the elements stay with numbers)
        d = gcd(d, *m); // * --> dereference the value stored in the address
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

// attribute -> marking functions and declarations with extra information (just like attributes in C# or annotations in Java)
#[test] // skipped by normal cargo build, but run by cargo test
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
