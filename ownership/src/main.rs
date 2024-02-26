/*
Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
Memory management models:

1. Garbage collection
Pros:
- Error free
- faster write time
Cons:
- No control over memory
- slower and unpredictable runtime performance
- larger program size

2. Manual memory management
Pros:
- Faster runtime
- Smaller program size
- Control over memory
Cons:
- Error prone
- Slower write time

3. Ownership model
Pros:
- Faster runtime
- Smaller program size
- Error free
- Control over memory
Cons:
- Slower write time

If you are writing a web app, you might want to use garbage collection, since you care more about error free code and faster write time.
If you are writing a game or any other low level program, you might want to use manual memory management, since you care more about faster runtime.
If you are writing a system program, you might want to use ownership model, since you care more about faster runtime, error free code and smaller program size.

Stack -> fixed size (calculated at compile time), fast, LIFO
Heap -> dynamic size, slower

----- Ownership rules -----
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    return String::from("hello");
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{}", s); // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x: i32 = 5;
    let y: i32 = x; // x is copied to y
                    // Rust has a special annotation called the Copy trait that we can place on types like integers, booleans and characters that are stored on the stack.

    // when you use &str, you create a reference to a static string stored in the program's executable, whereas when you use String, you create a heap allocated string
    // passing a reference to a function is called borrowing -> you can't modify the borrowed value, this you do to avoid unnecessary copying
    // in String, you have the ownership of the string (you can pass data around), whereas in &str, you have a reference to the string
    let s1 = String::from("hello"); // String is heap allocated and growable string type, whereas &str is a string slice which is a reference to a string
    let s2 = s1; // s1 is moved to s2
                 // println!("{}", s1); // this will throw an error, since s1 is no longer valid

    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3 is cloned to s4
    println!("s3 = {}, s4 = {}", s3, s4); // this will work

    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function and so is no longer valid here
                        // println!("{}", s); // this will throw an error, since s is no longer valid

    let x = 5;
    makes_copy(x); // x is copied to the function, so it's still valid here
    println!("{}", x); // this will work

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("{}", s1);

    let s2 = takes_and_gives_back(s1); // s1 is moved into the function, and the return value is moved into s2
    println!("{}", s2);
}
