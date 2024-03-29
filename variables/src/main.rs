fn my_function(x: i32) -> i32 {
    println!("Hey there! {x}");
    x
}

// rust is a statically typed language, which means that it must know the types of all variables at compile time
fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    // scaler types
    // - integer
    let x: i8 = 5;
    let y: u8 = 5;
    let x: i16 = 5;
    let y: u16 = 5;
    let x: i32 = 5;
    let y: u32 = 5;
    let x: i64 = 5;
    let y: u64 = 5;
    let x: i128 = 5;
    let y: u128 = 5;
    // - float
    let z: f64 = 5.0;
    let z: f32 = 5.0;
    // - boolean
    let t: bool = true;
    // - character
    let c: char = 'z';
    // compound types
    // - tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    // - array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first: i32 = a[0];

    println!("The value of x is {}", my_function(x));

    // control flow
    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is greater than 5");
    }

    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {result}");

    let mut number: i32 = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    let t = true;
    let f: bool = false;
    if t {
        println!("t is true");
    } else if f {
        println!("f is true");
    } else {
        println!("neither t nor f is true");
    }

    let y = {
        let mut x = 3;
        x += 1;
        x
    };
    println!("The value of y is: {}", y);

    let mut count = 0;
    'counting_up: loop {
        println!("Count is {count}");

        let mut remaining = 10;
        loop {
            println!("Remaining is {remaining}");

            if remaining == 9 {
                println!("Break");
                break;
            }
            if count == 2 {
                println!("Breaking loop 'counting_up");
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End of counting");
}
