// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}
#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    fn new(c: Color) -> Self {
        Self(c)
    }
}
#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor {
    fn new(c: Color) -> Self {
        Self(c)
    }
}
#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

fn print_shirt_color(color: ShirtColor) {
    println!("ShirtColor = {:?}", color);
}
fn print_shoes_color(color: ShoesColor) {
    println!("ShoesColor = {:?}", color);
}
fn print_pants_color(color: PantsColor) {
    println!("PantsColor = {:?}", color);
}
fn main() {
    print_shirt_color(ShirtColor::new(Color::Blue));
    print_shoes_color(ShoesColor::new(Color::Brown));
    print_pants_color(PantsColor::new(Color::Blue));
}
