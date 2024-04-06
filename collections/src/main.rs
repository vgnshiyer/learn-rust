use std::fmt::format;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    // vectors are collections that dynamically change size --> stored in heap memory

    /* Creating vectors */
    let mut v = Vec::new();
    v.push(1); // compiler infers the type of elements after this line
    
    let v = vec![1, 2, 3, 4 ,5]; // shortcut to create vectors

    /* Reading elements */
    let third: &i32 = &v[2]; // indexing (if the third element does not exists, the program will panic)

    let third: Option<&i32> = v.get(2); // using the get method --> returns a reference to the element in the Option enum (Returns a None when there is not such element, without panicking)
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    /* Ownership rules applies */
    // cannot have a mutable and immutable reference in the same scope

    let mut v = vec![1, 2, 3];

    let first = &v[0];

    v.push(4);

    // println!("The first element is {first}"); // as vectors may need to reallocated if size was full --> we need to store them in order in the memory

    /* Iterating over values */
    for i in &v { // we are borrowing instead of moving
        println!("{i}") // this is equivalent to println!("{}", *i) --> rust automatically dereferences it according to the context
    }
    // if we remove the &, for loop will take ownership of the vector and it will be lost from v after this line

    /* Hack to store multiple types in a vector */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.2),
        SpreadsheetCell::Text(String::from("Hello"))
    ];

    // String type 
    let s = String::from("some text");
    let mut s = "some text".to_string();

    let s1 = "here";
    s.push_str(s1); // the push_str method takes a string slice (borrows not take ownership) --> same as s + s1 (&str)
    println!("{s1}");

    let s2 = String::from("abc");

    // we can only add &str to a String type
    let s3 = s + &s2; // this statement takes ownership of s and appends a copy of s2 to it

    let s4 = format!("{s2}-{s3}"); // the format does not take owner ship --> it borrows a reference

    // type string cannot be indexed in rust --> because unicode characters are actually stored as byte vector internally.. 

    // best way to deal with strings is to be specific about it (whether you want chars or bytes)

    for c in s4.chars() {
        println!("{c}");
    }
    for c in s4.bytes() {
        println!("{c}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("black"), 20);
    // hashmaps are stored in the heap memory. They are homogenous, all keys and values must be of same type.

    let score = scores.get("blue").unwrap_or(&0); // if not present, give 0 --> notice that we borrow and not take ownership of the value

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // hashmap takes ownership of types like string.. not for i32

    // overwriting a value in a hashmap
    scores.insert(String::from("blue"), 0);

    println!("{:?}!", scores);

    // adding a key and value only if a key isnt present

    scores.entry(String::from("yellow")).or_insert(50); // or_insert returns a mutable reference to the value --> if not exists, enters 50 for the key and returns mutable reference

    // counting word occurences in a sentence
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}!", map);
}
