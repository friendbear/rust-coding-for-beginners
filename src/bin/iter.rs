use core::num;
// [src/bin/iter.rs:7] plus_one = [
//     2,
//     3,
//     4,
//     5,
//     6,
// ]
// [src/bin/iter.rs:11] new_numbers = [
//     1,
//     2,
//     3,
// ]
// [src/bin/iter.rs:15] find_me = Some(
//     3,
// )
// [src/bin/iter.rs:18] count = 5
// [src/bin/iter.rs:21] last = Some(
//     5,
// )
// [src/bin/iter.rs:24] max = Some(
//     5,
// )
// [src/bin/iter.rs:27] min = Some(
//     1,
// )
// [src/bin/iter.rs:30] take = [
//     1,
//     2,
//     3,
// ]`
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let plus_one: Vec<_> = numbers.iter().map(|&v| v + 1).collect();
    dbg!(plus_one);

    let numbers = vec![1, 2, 3, 4, 5];
    let new_numbers: Vec<_> = numbers.iter().filter(|&v| *v <= 3).collect();
    dbg!(new_numbers);

    let numbers = vec![1, 2, 3, 4, 5]; // 🔨
    let find_me: Option<&i32> = numbers.iter().find(|&&x| x == 3);
    dbg!(find_me);

    let count = numbers.iter().count();
    dbg!(count);

    let last = numbers.iter().last();
    dbg!(last);

    let max = numbers.iter().max();
    dbg!(max);

    let min = numbers.iter().min();
    dbg!(min);

    // 🔨
    let take: Vec<_> = numbers.iter().take(3).collect::<Vec<_>>();
    dbg!(take);
}
