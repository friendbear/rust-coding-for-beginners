
fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {

    let add = |a: i32, b: i32| -> i32 {
        a + b
    };

    let sum1 = add_fn(1, 1);
    let sum2 = add(1, 1);
    assert_eq!(sum1, sum2);
}