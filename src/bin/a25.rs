// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter
trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}
struct Square {
    side: i32,
}
struct Triangle {
    side: (i32, i32, i32),
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side + 4
    }
}
impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.side.0 + self.side.1 + self.side.2
    }
}
fn print_perimeter(shape: impl Perimeter) {
    let perimeter = shape.calculate_perimeter();
    println!("permimeter = {:?}", perimeter)
}
fn main() {
    let square = Square { side: 5 };
    let triangle = Triangle { side: (2, 3, 4) };

    print_perimeter(square);
    print_perimeter(triangle);
}
