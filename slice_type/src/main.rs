fn main() {
    // slice is always reference to a part of a String, or array or any collection data structure

    // a slice data structure stores the starting index and the length of the slice

    let mut s = String::from("Hello World");

    // type that signifies String slice is &str
    let slice: & str = &s[0..5]; // immutable borrow
    let slice: &mut str = &mut s[0..5]; // mutable borrow

    let first = first_word(&s);

    // s.clear(); --> Error caught during compile time: clear does a mutable borrow, but we have an immutable borrow happening before this which is being used in the print statement below 

    println!("first word is {first}");

    let a = "hello world"; // this is an string slice
    let b = &String::from("Hello world"); // this is a String reference

    /*
    str is a string slice, which is a reference to a string. It is immutable seq of utf8 bytes. It can only be used in borrowed fashion.
    String is a String type, which is growable and mutable.

    For variable `a` above, the str slice is stored in the program's binary (stack memory) and is removed as the program ends.
    For variable `b` as we are taking a reference to the String, the scope itself becomes the owner and `b` has borrowed its immutable reference.
    */

    let arr: [usize; 3] = [1, 2, 3];

    let slice: &[usize] = &arr[..2];
}

fn first_word(s: &String) -> &str {
    let s_bytes = s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}