use core::panic;
use std::{error::Error, fs::{self, File}, io::{self, ErrorKind, Read}};

fn main() {
    // there are two types of errors in Rust 
    // 1. Recoverable errors --> returns Result<T, E>
    // 2. Unrecoverable errors --> runs the panic! macro

    // panic! macro empties the stack and quits the program.
    // panic = 'abort' in profile section of cargo.toml file allows aborting without cleaning up, eventually reducing the size of the binary.

    let v = vec![1, 2, 3];

    v[11]; // panics as index is out of bounds (in C, we get some random data)

    // the result enum
    enum Result<T, E> { // generic types T and E
        Ok(T), // Variants Ok and Err
        Err(E)
    }

    let file_open_result = File::open("hello.txt");
    // let file = match file_open_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file {:?}", error),
    // };
    // println!("{:?}", file);

    /* digression: printing formatters 
    {} -> default formatter 
    {:?} -> debug formatter --> structs and enums --> types that implement the Debug trait (#[derive(Debug)]])
    {:#?} -> similar to {:?}, but more human readable
    */

    let file = match file_open_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating new file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    
    let file = File::open("hello.txt").unwrap(); // unwrap method internally implements the match expression, sends the file if success, else panics

    let file = File::open("hello.txt")
        .expect("hello.txt is not present"); // expect methods allows us to control the error message to the panic! macro


}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = match File::open("username.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// with ? operator
fn read_username_from_file1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // similar to unwrap, ? operator returns the value inside Ok if success and if Err, it returns the whole Err variant with the value **to the calling code**

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // ? operator can only be used inside functions
    // ? operator returns Err(e) directly to the function caller
    // ? operator uses the From trait to convert the Error type according to the current function return type -> without needing to add any extra code
}

// shorter version
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// even shorter way --> using the fs module
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}