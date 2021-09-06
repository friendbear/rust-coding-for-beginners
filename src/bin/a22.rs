// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn clamp_lower() {
        let result = clamp(10, 100, 10000);
        let expected = 100;
        assert_eq!(result, expected, "should be 100")
    }

    #[test]
    fn clamp_upper() {
        let result = clamp(50000, 100, 10000);
        let expected = 10000;
        assert_eq!(result, expected, "should be 10000")
    }

    #[test]
    fn check_div() {
        let result = div(10, 2);
        let expected = Some(5);
        assert_eq!(result, expected, "should be 5");
    }

    #[test]
    fn check_div_zero(){
        let result = div(10, 0);
        let expected = None;
        assert_eq!(result, expected, "cannot divide zero");

    }

    #[test]
    fn check_concat() {
        let result = concat("first", "second");
        let expected = String::from("firstsecond");

        assert_eq!(result, expected, "should be equal")
    }

}
