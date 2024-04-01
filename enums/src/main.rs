use std::any::Any;

// enums are a way of encapsulating variants of the same type into a data structure
#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8), // you can directly put data into a variant
    V6(String),
}

// enums can also have methods
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: u32, y: u32},
    Write(String)
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

// Option<T> enum: in other programming languages we have the problem where we might possibly assume a variable has some value while it may not have one (it may be null). Rust provides the Option<T> enum with attributes Some that represents that there is some value of type T, and a None variant that represents a null value. This forces the user to implement the code for the case where the value may by null and value may be of some type T (eg. extract the T type). At compile time!!

fn first_occurence(sentence: &str) -> Option<&str> { // this may either return 
    let search = "hello";
    for word in sentence.split_whitespace() {
        if word == search {
            return Some(word);
        }
    }
    None
}

fn main() {
    let home_addr = IpAddrKind::V4(127, 0, 0, 1);
    let office_addr: IpAddrKind = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let first = first_occurence("hello world");
    match first {
        Some(word) => println!("{}", word),
        None => println!("No occurrence found."),
    }

    let none = first_occurence("invalid string");
    match none {
        Some(word) => println!("{}", word),
        None => println!("No occurrence found."),
    }
}