use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("Advent of Code");
    println!("Day 3, Part 1");
    println!("Kent West - 9June.2025");

    // Get data.
    let data = read_to_string("src/data.txt").expect("Error reading input data file.");

    // Define the regex for which we're searching.
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut total = 0;

    // Find all matches for that regex.
    for mat in re.find_iter(&data) {
        // For each match, strip all but the "number,number" pair portion
        let pair = &mat.as_str()[4..(&mat.as_str().len() - 1)];
        let pair: Vec<&str> = pair.trim().split(',').collect();
        let x: i32 = pair[0].parse().expect("Error converting ASCII to i32");
        let y: i32 = pair[1].parse().expect("Error converting ASCII to i32");
        total += (x * y);
    }
    println!("Total = {}", total);
}
