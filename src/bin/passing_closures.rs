/// dyn keyword is used to create a dynamic type
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}
fn main() {
    let name = "T Kumagai";

    let add = Box::new(move |a, b| {
        println!("adding a number for {}!", name);
        a + b
    });
    let sub = |a, b| a - b;

    println!("{}", math(2, 2, add));

    let sub: Box<_> = Box::new(sub);
    println!("{}", math(2, 2, sub));
}
