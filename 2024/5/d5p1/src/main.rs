const COMMENTS: &str = r#"
    Sorts pages for printing.

    Advent of Code
    Day 5, Part 1
    Kent West - 23.June.2025
"#;

use std::fs::read_to_string;

fn main() {
    println!("{}", COMMENTS);

    // Read input data file into "data" String.
    let filename = "src/data.txt";
    let err_msg = format!("Error reading data file \"{}\".", filename);
    let mut data = read_to_string(filename).expect(&err_msg);

    // Trim whitespace from beginning/end of "data".
    data = data.trim().to_string();

    // Split "data" into an iterator over the lines, and collect them into a vector.
    let sections: Vec<&str> = data.split("\n\n").collect();

    // Convert each section to a vector of slices.
    let ordering: Vec<&str> = sections[0].lines().into_iter().collect();
    let updates: Vec<&str> = sections[1].lines().into_iter().collect();

    println!("\n\tSection 1:");
    for line in ordering {
        println!("{}", line);
    }

    println!("\n\tSection 2:");
    for line in updates {
        if is_order_correct(line) {
            println!("CORRECT! - {}", line);
        } else {
            println!("         - {}", line)
        }
    }
} // end of main()

fn is_order_correct(update: &str) -> bool {
    // Split &str on commas to convert the &str to a Vec<&str>.
    let page_nums: Vec<&str> = update.split(",").into_iter().collect();

    for page_num in &page_nums {
        println!("Page Number = {}", page_num);
    }
    if page_nums[0] == "75" { true } else { false }
} // end of is_order_correct()
