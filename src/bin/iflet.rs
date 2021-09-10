fn main() {
    let maybe_user = Some("T Kumagai");

    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    }
}
