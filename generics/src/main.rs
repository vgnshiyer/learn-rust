fn largest(list: &Vec<i32>) -> i32 {
    let mut largest = list[0];

    for i in list {
        if largest < *i {
            largest = *i;
        }
    }

    largest
}

fn largest_char(list: &Vec<char>) -> char {
    let mut largest = list[0];

    for i in list {
        if largest < *i {
            largest = *i;
        }
    }

    largest
}

// generic functions
fn largest_cmn<T>(list: &Vec<T>) -> &T { // we first declare the type parameter (T) in angle bracket after the function declaration
    let mut largest = &list[0];

    for i in list {
        // if largest < *i { // this wont work as rust is not sure how to implement comparison on unknown types
        //     largest = *i;
        // }
    }

    largest
}

// generic structs
struct Point<T> {
    x: T, // whatever we pass as type for x, will be inferred for T --> y has to be of the same type
    y: T
}

// generic structs with multiple types
struct Point1<T, U> {
    x: T,
    y: U,
}

// generic enums
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

// Generics in method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> { // this method will only be valid of points of type = f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) +self.y.powi(2)).sqrt()
    }
}

/*
Generics dont affect runtime of the program. Rust checks what all types are used and creates approriate implementations (performs duplication) for each type during compile time --> process called monomorphization
*/

fn main() {
    // Generics allow you to define functions, structs, enums, and methods that work with any type.
    // avoids duplication of code

    // de-duplicating a largest number in an array --> create a function and call it against different vectors
    let list = vec![1, 2, 3];
    println!("the largest number is: {}", largest(&list));
    
    let list = vec!['a', 'b', 'c'];
    println!("the largest char is: {}", largest_char(&list));
}
