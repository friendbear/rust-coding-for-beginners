// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    let my_integer = 6;
    if my_integer > 5 {
        println!("gt 5");
    } else if my_integer < 5 {
        println!("lt 5");
    } else if my_integer == 5 {
        println!("eq 5");
    }
}
