use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("Advent of Code");
    println!("Day 3, Part 2");
    println!("Kent West - 9June.2025");

    // Get data.
    let data = read_to_string("src/data.sam").expect("Error reading input data file.");

    // Define the regex for which we're searching.
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    println!("{}", data);

    // Find all matches for that regex and put them in a vector,
    // stripping away all but the "number,number" pairs in the match.
    for mat in re.find_iter(&data) {
        let pair = &mat.as_str()[4..(&mat.as_str().len() - 1)];
        println!("{}", pair);
    }
}
