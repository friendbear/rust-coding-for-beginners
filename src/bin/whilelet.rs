use core::num;

fn main() {
    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop:{:?}", i);
        data = None;
    }

    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter(); // ⚡ mut
    while let Some(num) = number_iter.next() {
        println!("num = {:}", num);
    }

    println!("done");
}
