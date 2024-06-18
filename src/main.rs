

pub mod disc;

use crate::disc::length_of_last_word;


fn main() {
    println!("Hello, world!");

    length_of_last_word("sredtjnh".to_string());

    let g: i32 = factorial(10);
    println!("{g}");
}

fn factorial(x: i32) -> i32
{
    if x == 0 { 1 } else { x * factorial(x - 1) }
}