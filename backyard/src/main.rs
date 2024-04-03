use crate::garden::vegetables::Aspargus;

/*
Terms
Crate: basic compilation unit. Binary crate --> executables, Library crate --> reusable functionality
Module: namespace containing definitions, functionalities --> used to organiza code
Package: A set of crates, contains cargo.toml.

Module structure
root(crate)
 |
 - garden/mod.rs
    |
    - vegetable/mod.rs or simply vegetable.rs
*/

pub mod garden;

fn main() {
    let plant = Aspargus {};
    println!("I'm growing {:?}!", plant);
}
