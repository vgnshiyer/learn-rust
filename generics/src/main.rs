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
        if largest > *i {
            largest = *i;
        }
    }

    largest
}

fn main() {
    // Generics allow you to define functions, structs, enums, and methods that work with any type.
    // avoids duplication of code

    // de-duplicating a largest number in an array --> create a function and call it against different vectors
    let list = vec![1, 2, 3];
    println!("the largest number is: {}", largest(&list));
    
    let list = vec!['a', 'b', 'c'];
    println!("the largest char is: {}", largest_char(&list));
}
