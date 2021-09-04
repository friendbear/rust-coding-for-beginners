// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
use std::io::{self, Read};

fn get_input() -> io::Result<String> {

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
fn main() {
    let mut all_input = vec![];
    let mut time_input = 0;

    while time_input < 2 {
        match get_input() {
            Ok(word) => {
                all_input.push(word);
                time_input += 1;
            },
            Err(e) => println!("error: {:?}", e),
        }
    }

    for input in all_input {
        println!("Original: {:?}, capitalized: {:?}", input, input.to_uppercase());
    }
}
