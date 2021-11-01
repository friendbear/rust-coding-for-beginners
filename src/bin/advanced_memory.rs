#[derive(Debug)]
enum Answer {
    Yes,
    No,
}

fn main() {}

#[cfg(test)]
#[test]
fn memory_advanced() {
    let yes = Answer::Yes;
    let yes_heap: Box<Answer> = Box::new(Answer::Yes);
    let yes_stack = *yes_heap;
}
