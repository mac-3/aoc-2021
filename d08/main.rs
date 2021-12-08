mod a;
mod b;
mod digit;

use crate::{a::part_a, b::part_b};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("input.txt")?)?;
    println!("A: {}", part_a(&input));
    println!("B: {}", part_b(&input));
    Ok(())
}
