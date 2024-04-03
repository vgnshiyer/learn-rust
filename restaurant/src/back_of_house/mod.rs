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