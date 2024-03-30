/*
Ownership rules:
1. Each value has an owner.
2. Only one owner per value.
3. When owner goes out of scope, the value is dropped.
*/

fn main() {
    // scope : range within a program for which a value is valid
    {                    // s is not valid here
        let s = "Hello";  // s is valid here

    }                    // validity of s ends here (scope is over)

    // Difference between string literal and String type
    // string literal is immutable.. Size is known and hence, it is stored onto the stack
    // String type is unknown in size and hence is stored in the heap

    // In Garbage collected languages, the compiler automatically deallocates this memory when it is not being used anymore.
    // In languages without GC, we need to decide when to deallocate (not too early, or not at all--> waste memory, 
    
    // In rust, memory is automatically deallocated when the variable that owns it goes out of scope.

    let x = 5;
    let y = x;
    // both vars with value 5 are pushed onto stack --> because this implements the 'copy' trait

    let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("value of s1: {s1}"); --> here, the 'move' trait is called because we are dealing with the String type and the heap memory..
    // s1 is out of score as soon as it is moved to s2. s2 is the new owner of "Hello"
    let s2 = s1.clone(); // --> this implements the 'copy' trait and a deep copy is made2
    
    // same is followed when passing values to functions

    takes_ownership(s2); // s2 is moved to fn: takes_ownership --> s2 is no longer valid below this line

    makes_copy(x); // x is moved to fn: makes_copy, but since x is an i32 type, it implements the 'copy' trait and u can still use x below this

    let s = gives_ownership(); // s comes into scope from fn: gives_ownership

    let t = takes_and_gives_back(s); // s is moved to fn: takes_and_gives_back and moved back to t

    // s is not valid here
} // scope for x, t, s2, s1 end here

fn takes_ownership(s: String) { // s comes in scope 
    println!("{s}"); // s is borrowed from the caller
} // scope ends here, 'drop' is called

fn makes_copy(x: i32) { // x comes in scope
    println!("{x}");
} // scope ends here

fn gives_ownership() -> String {
    let s = String::from("yours"); // s comes in scope
    s // s is moved to caller
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// moral: Rust knows at compile time, when to deallocate memory for variables from the heap due to the ownership model.. --> Therefore it does not require a garbage collector.