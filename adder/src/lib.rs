pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

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

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Self { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value: {a}");
    10
}

// to run all tests use: cargo tes
// to run all tests consecutively use: cargo test -- --test-threads=1
// to see printed values for passing tests use: cargo test -- --show-output
// to run specific test: cargo test add (all test whose names contain 'add' will be run)
// to run only ignored tests: cargo test -- --ignored
// to run all tests including the ignored ones use: cargo test -- --include-ignored
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = String::from("Karol");
    //     assert!(
    //         result.contains("Carol"),
    //         "greeting did not contain name, value was `{result}`" // provide custom failure message
    //     );
    // }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_v2() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, prints_and_returns_10(50));
    }

    // #[test]
    // fn this_test_will_fail() {
    //     assert_eq!(5, prints_and_returns_10(42));
    // }

    #[test]
    fn add_v1() {
        assert_eq!(add(10, 20), 30);
    }

    #[test]
    fn add_v2() {
        assert_eq!(10, add(1, 9));
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}
