#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn greet(name: &str) -> String {
    format!("Hey {name}")
}

struct guess {
    value: i32,
}

impl  guess {
    fn new(value: i32) -> guess {
        if value < 1 || value > 100 {
            panic!("Enter a value between 1 and 100");
        }
        guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn large_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 2,
            height: 3
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 2,
            height: 3
        };

        assert!(!smaller.can_hold(&larger));
    }

    // example: default optional arguments to assert
    #[test]
    fn greeting_contains_name() {
        let name = "vignesh";
        let result = greet(name);
        // assert!(result.contains("vigu"), "Greeting did not contain {}", name);
    }

    // example: should_panic
    #[test]
    #[should_panic(expected = "less than 100 pls")]
    fn greater_than_100() {
        guess::new(100);
    }
}
