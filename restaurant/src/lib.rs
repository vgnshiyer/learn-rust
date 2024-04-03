/*
module tree

crate(root)
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super --> go to parent [crate in this case] (cd ..) {can use super as many times to go higher in the tree}
    }

    fn cook_order() {}

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        fruit: String
    }

    // this function is needed as we can never create a breakfast instance as the fruit field is private.
    impl  Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("Anaar")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// can bring a function or module in scope using the `use` keyword
use front_of_house::hosting::add_to_waitlist;
// can add pub in front to make it available for modules using this module (crate) --> this is called hosting or re-exporting

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // parent modules cannot access functions, structs of their child modules. The other way round is possible. In order to allow parents use their child module functionality, we need to add pub keyword

    // relative path
    self::front_of_house::hosting::add_to_waitlist();

    // in scope (just like symlink in linux filesystem or shortcut in windows)
    add_to_waitlist();

    // altho it is better to bring the parent module of the function/struct/enum you want to use in scope rather than the actual thing --> makes it more readable and says that the function is not a local function (This method is called idiomatic reference)

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("{:?}!", meal);

    // enums dont need to make individual variants public unlike structs
    let ap1 = back_of_house::Appetizer::Soup;
    let ap2 = back_of_house::Appetizer::Salad;
}