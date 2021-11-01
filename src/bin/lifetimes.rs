#[derive(Debug)]
enum Answer {
    Yes,
    No,
}

#[derive(Debug)]
struct Form<'a> {
    question: &'a Answer,
}

fn main() {
    let form;
    let answer = Answer::Yes;
    {
        form = Form { question: &answer };
    }

    println!("{:?}", form);
}

#[derive(Debug)]
struct Quiz {
    question: Answer,
}

fn get_first_question<'a>(quiz_1: &'a Quiz, quiz_2: &'a Quiz) -> &'a Answer {
    &quiz_1.question
}
