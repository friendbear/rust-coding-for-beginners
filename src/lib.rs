pub mod group;
mod helpers;
use helpers::{print_again, print_from_helper};

pub fn print_firom_lib() {
    print_from_helper();
    println!("hello from lib");
    print_again();

    group::g1::g1_hello();
}
