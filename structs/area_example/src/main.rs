fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area3(rect: &Rect) -> u32 {
    rect.width * rect.height
}

// we need to implement the Debug trait to be able to print this
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

// methods
impl Rect {
    fn area(self: &Self) -> u32 { // Self --> Rect (can also be written with shorthand syntax &self) [We can also take ownership by removing the & symbol]
        self.width * self.height
    }
}

fn main() {
    let width: u32 = 3;
    let height: u32 = 4;

    println!("area: {}", area1(width, height)); // too verbose and not clear

    // with tuples
    let rect1 = (3, 4);

    println!("area: {}", area2(rect1)); // better, but not so clear (.0 .1 ?) --> would be nice to distinguish between width and height

    // structs
    let rect2 = Rect {
        width: 3,
        height: 4
    };

    println!("rect: {:?}", rect2); // {:?} informs println! that we want to use output format called Debug
    println!("rect: {:#?}", rect2); // {:#?} prettier Debug formatting
    println!("area: {}", area3(&rect2));

    // can also use the dbg! macro (dbg! macro takes ownership and returns it back unlike println! macro which just takes the reference)
    // it also prints the line number and file
    let mut rect3 = Rect {
        width: dbg!(2 * 3),
        height: 2
    };

    rect3 = dbg!(rect3);

    println!("area: {}", rect3.area());

    /*
    Output:

    area: 12
    area: 12
    rect: Rect { width: 3, height: 4 }
    rect: Rect {
        width: 3,
        height: 4,
    }
    area: 12
    [area_example/src/main.rs:44:16] 2 * 3 = 6
    [area_example/src/main.rs:48:5] rect3 = Rect {
        width: 6,
        height: 2,
    }
    */
}
