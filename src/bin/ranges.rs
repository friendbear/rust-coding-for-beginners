use core::num;

fn main() {
    let num_range_eq = 1..=4; // => RangeInclusive<i32> ✅
    dbg!(num_range_eq);

    let num_range = 1..5; // => Range<i32> ✅
    dbg!(num_range);

    for num in 1..5 {
        println!("{:?}", num);
    }

    for c in 'a'..='z' {
        println!("{:?}", c);
    }
}
