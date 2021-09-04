fn maybe_num() -> Option<i32> {
    Some(1)
}

fn maybe_word() -> Option<String> {
    Some(String::from("maybe_word"))
}

fn main() {

    let word_length = maybe_word()
        .map(|s| s.len())
        .map(|len| len * 2);

    println!("word_length: {:?}", word_length.unwrap());
}