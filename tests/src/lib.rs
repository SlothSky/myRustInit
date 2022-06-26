#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(init: i32) -> i32 {
    init + 2
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { _value: value }
    }
} 

fn _print_and_return_10(a: i32) -> i32 {
    println!("THIS LINE IS ONLY PRINTED IF THE ACCORDING TEST CASE FAILS {}", a);
    10
}

#[cfg(test)]
mod tests {
    // required for getting the outer scope (out of module tests) into the tests module
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        // assert_eq macro asserts the input results in the expected output
        assert_eq!(result, 4);
    }

    #[test]
    fn somthing_does_not_work() {
        let _x = "irrelevant";
        panic!("x is irrelevant, as this stuff here panics");
    }

    // the assert macro panics, if its parameter does not resolve to true 
    // if the macro detects that those two do not match, it calls the panic macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 6,
        };
        let smaller = Rectangle {
            width: 4,
            height: 2,
        };
        assert!(larger._can_hold(&smaller));
    }

    // negative test 
    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 9,
            height: 2,
        };
        let smaller = Rectangle {
            width: 7,
            height: 1,
        };
        assert!(!smaller._can_hold(&larger));
    }

    // checking via assert_eq & assert_ne
    #[test]
    fn function_adds_two() {
        assert_eq!(89, add_two(87));
    }

    #[test]
    fn function_does_not_add_two() {
        assert_ne!(233, add_two(232));
    }

    #[test]
    fn add_additional_information() {
        let larger = Rectangle {
            width: 9,
            height: 2,
        };
        let smaller = Rectangle {
            width: 7,
            height: 1,
        };
        assert_eq!(
            smaller._can_hold(&larger),
            true,
            "This rectangle {} cannot fit in the smaller rectangle",
            "THIS SHOULD BE A VARIABLE"
        )
    }

    #[test]
    #[should_panic]
    fn greater_than_100_guess() {
        Guess::new(200);
    }

    #[test]
    // only a substring is required for the expected panic string
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100_guess_with_expected() {
        Guess::new(200);
    }

    // with Results, an Ok() is returned when the test passes and an Err() is
    // returned when the test failes. The Err String is in this case reused as test error verdict
    #[test]
    fn returning_error_strings() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("In some universe, 2 + 2 does not equal 4"))
        }
    }
    // if assertion for an Err is required → use assert!(value.is_err())

    // first test will pass → thus no output of the functions println! macro
    #[test]
    #[ignore]
    fn passing_test_has_no_stdout_output() {
        let result = _print_and_return_10(10);
        assert_eq!(result, 10);
    }

    // second test will fail → thus println! macro will have output to stdout
    #[test]
    fn failing_test_has_stdout_output() {
        let result = _print_and_return_10(10);
        assert_eq!(result, 8);
    }
}









