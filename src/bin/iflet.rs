enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let maybe_user = Some("T Kumagai");

    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    }

    let color = Color::Blue;
    if let Color::Red = color {
        println!("its red!");
    } else {
        println!("its not red");
    }
}
