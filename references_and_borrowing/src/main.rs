fn main() {
    // when we want to just do a particular action on a variable and return it back, there is a better method

    // borrowing --> send a reference (represented by '&') to the function : a pointer to the actual data instead of 'moving' and passing the ownership

    let mut s = String::from("Hello");

    let s_len = calculate_len(&s); // pass a reference of s to fn: calculate_len

    println!("{s} has length {s_len}");

    // mutable references
    change(&mut s); // condition with mutable references --> They can only be borrowed once
    // this is done to avoid race conditions --> dificult to diagnose

    println!("{s}");

    // few anti-patterns
    
    // Borrow twice in the same scope
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1} {r2}");

    // Borrow immutable and mutable in the same scope
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;

    // println!("{r1} {r2} {r3}")

    // Can borrow as mutable if immutables are used before borrowing
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    let r3 = &mut s;
    println!("{r3}");
    
    // Rust compiler ensures that a value doesnt go out of scope before the reference does

    // How rust compiler avoids dangling references (reference to a memory where the data no longer exists)
    // let reff = get_ref();
    // solution is to simply pas ownership to the string
    
} // scope of s and s_len ends here

fn calculate_len(s: &String) -> usize { // s is merely borrowed here
    s.len()
}

fn change(s: &mut String) {
    s.push('!');
}

// fn get_ref() -> &String { // reference
//     let s = String::from("Hello");
//     &s // return reference
// } // but s goes out of scope and dropped before it is used by the reference --> Compiler catches this