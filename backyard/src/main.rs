use crate::garden::vegetables::Aspargus;

/*
Terms
Crate: basic compilation unit. Binary crate --> executables, Library crate --> reusable functionality
Module: namespace containing definitions, functionalities --> used to organiza code
Package: A set of crates, contains cargo.toml.

Module structure
root(crate)
 |
 - garden/mod.rs (mod contains the code for the module)
    |
    - vegetable/mod.rs or simply vegetable.rs (vegetable contains the code for the module)

Can also add module code directly in curly braces instead of creating a new file.
*/

pub mod garden;

fn main() {
    let plant = Aspargus {};
    println!("I'm growing {:?}!", plant);
}
