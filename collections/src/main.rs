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
}
