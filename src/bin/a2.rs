// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
// * Use a function to display the result
fn disoplay_result(res: i32) {
    println!("{:?}", res)
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let result = sum(2 , 2);
    disoplay_result(result)
}
