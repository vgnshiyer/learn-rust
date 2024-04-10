// every reference has a lifetime --> scope for which the reference is valid --> they are inferred by the compiler

// main goal of lifetime is to prevent access of data that is not allowed

// the generic lifetime 'a does not change the lifetimes of the variables x and y or the return val, rather it just specifies that the lifeime of return val must be as long as the shortest lifetime of the two variables x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} // simple words: the return val cannot outlive the parameters

// lifetimes in structs
struct important_excerpt<'a> {
    part: &'a str
}

fn main() {
    // general rule: lifetime of a borrowed value must be greater than the lifetime of the borrower

    // eg.
    /*
    let r;                 ---------+----'a
    {                      --+---'b |
        let x = 5;           |      |
        r = &x;            --+      |
    }                               |
    println!("{r}")        ---------+
     */

    // the lifetime 'b of the borrowed(x) is shorter than the lifetime 'a of the borrower(y)

    // valid example
    let x = String::from("long");
    let z;

    {
        let y = String::from("longer");
        z = longest(x.as_str(), y.as_str());
        println!("{}", z); // the lifetime of z is as long as the shortest lifetime (y)
    }

    // invalid example
    let a = String::from("longer");
    let c;

    {
        let b = String::from("long");
        c = longest(a.as_str(), b.as_str()); // b doesnt live long enough
    } // b's scope ends here

    // even if the return value is a and it is still in scope, the compiler fails --> as we told the compiler that the lifetimes of return value will be as long as the shortest of lifetimes of x and y
    println!("{}", c); // the return values lifetime should not be greater than the shortest lifetime of (a, b)

    let novel = String::from("sentence 1. sentence 2");
    let first_sentence = novel.split('.').next().expect("No first sentence");
    let excerpt = important_excerpt {
        part: first_sentence
    }; // first sentence lives as long as except: Valid

    // You need to specify lifetimes for functions and structs that use references

    // static lifetimes --> specified for a reference that exists for the entire duration of the program. It gets stored in the program's binary
    
    // detour: example of data on binary, stack and heap
    let var = "hello";
    // string literal "hello" is known at compile time.. Therefore it gets stored in the program's binary (data block)
    // the variable var which is a reference to this string is stored in the stack memory.

    let var = String::from("Hello");
    // this is a dynamic string type which is stored in the heap memory.
    // the variable var is a reference to this string type and is stored in the stack memory.
}