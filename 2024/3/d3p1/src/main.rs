// Sums up valid multipliers.
// Result with test data should be  161

use std::fs::*;

const INPUT_FILE: &str = "src/input.dat";

fn main() {
    println!("Advent of Code");
    println!("Day 3, Part 1");
    println!("Kent West - 9June.2025");

    // Get data from input file, as a String.
    let err_msg = format!("Error reading \"{}\".", INPUT_FILE);
    let data_string: String = read_to_string(INPUT_FILE).expect(&err_msg);

    // Initialize a vector to hold the pairs.
    let pairs: Vec<String> = Vec::new();
    let data_chars = data_string.chars();
    println!(
        "The length of the string [as chars] is: {}",
        &data_chars.clone().count()
    );
    println!("The chars are:");
    for (x, ch) in data_chars.enumerate() {
        println!("{}. {}", x, ch);
    }
    println!();
}
