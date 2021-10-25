#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("cannot be zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;

    a / b
}

fn main() {
    let data = vec![NeverZero::new(0), NeverZero::new(10)];
    for v in data {
        match v {
            Ok(nz) => println!("{:?}", divide(10, nz)),
            Err(e) => println!("{:?}", e),
        }
    }
}
