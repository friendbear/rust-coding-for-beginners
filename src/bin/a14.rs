// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    name: String,
    fav_color: String,
    age: i32,
}
fn main() {

    let persons = vec![
        Person {
            name: "hoge".to_owned(),
            fav_color: String::from("green"),
            age: 32,
        },
        Person {
            name: "Anna".to_owned(),
            fav_color: String::from("purple"),
            age: 9,
        },
        Person {
            name: "Katie".to_owned(),
            fav_color: String::from("blue"),
            age: 10,
        }
    ];

    let under10_age_persons = persons.iter().filter(|&x| x.age <= 10);
    for person in under10_age_persons {
        println!("{:?}", person.age);
    }

}
