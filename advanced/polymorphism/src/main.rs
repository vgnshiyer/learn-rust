trait MakesSound {
    fn sound(&self) -> String;
}
struct Dog {
    name: String
}

impl MakesSound for Dog {
    fn sound(&self) -> String {
        "Woof".to_string()
    }
}

struct Cat {
    name: String
}

impl MakesSound for Cat {
    fn sound(&self) -> String {
        "Meow".to_string()
    }
}

// compile-time polymorphism example (give information about the datatype during compile time)
fn make_sound<T: MakesSound>(animal: &T) -> String {
    format!("Animal makes sound: {}", animal.sound()) // T with MakesSound trait will have a sound() method
}

// run-time polymorphism example (the type of the object that will be passed can only be known at runtime)
fn make_sound_dyn(animal: &dyn MakesSound) -> String {
    format!("Animal makes sound: {}", animal.sound())
}

// run-time is more concise and readable, but it not optimized as the implementation need to be stored in the heap memory.
// performance impact: trait objects store pointers to actual data and pointers to actual method implementations. this pointer lookup that happens dynamically based on the type of the object is where the performance impact is -> these references are stored in the heap
// heap memory is slow and has a high lookup time than stack.

// run-time polymorphism example (with Box)
fn make_sound_dyn_with_box(animal: Box<dyn MakesSound>) -> String {
    format!("Animal makes sound: {}", animal.sound())
}

fn main() {
    let dog = Dog {
        name: "Brok".to_string()
    };

    let cat = Cat {
        name: "Tom".to_string()
    };

    // a.k.a static dispatch
    println!("{}", make_sound(&dog));
    println!("{}", make_sound(&cat));

    // Compile time polymorphism is more optimized as it tells the compiler exactly what kind of data it should expect
    // with this, the compiler will create different implementations for the `make_sound` function for all the data types that implement the MakeSound trait.
    // There will be two function definitions (concrete implementations: 1 for Dog, 1 for Cat) inside the stack memory of the program.

    // runtime polymorphism example
    // create a vector of reference to a trait object
    let animals: Vec<&dyn MakesSound> = vec![&dog, &cat];
    for animal in animals {
        // a.k.a dynamic dispatch
        println!("{}", make_sound_dyn(animal));
    }

    // Box -> smart pointer --> says the compiler that the data inside must be stored in the heap instead of the stack
    // we use this for getting the ownership of the dynamic trait object

    // In an ideal world you must store everything on the stack because it is much faster.
    // However, to store something on the stack, you need to tell the compiler the size of the object to store in the stack.
    // In some cases, (for example a Trait object) the size of the object can only be known during runtime. Compiler has no way to know its size.
    // only solution is to store it in the heap. a.k.a dynamic dispatch
    // There are two ways to do this:
    // 1. with trait object references  (example above) : Store object in stack and store reference to it in heap
    // 2. with Box : Store the object itself in heap and get the ownership to it

    let animals_2: Vec<Box<dyn MakesSound>> = vec![
        Box::new(Dog { name: "Brok".to_string() }),
        Box::new(Cat { name: "Tom".to_string() })
    ];

    for animal in animals_2 {
        println!("{}", make_sound_dyn_with_box(animal));
    }

}
