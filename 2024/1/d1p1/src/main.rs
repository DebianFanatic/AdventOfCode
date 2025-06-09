use std::fs::read_to_string;

fn main() {
    println!("Advent of Code");
    println!("Day 1, Part 1");
    println!("Kent West - 8.June.2025");

    // Read the data file as one long continuous String.
    const DATA_FILE: &str = "src/data.txt";
    let data = match read_to_string(DATA_FILE) {
        Ok(contents) => contents,
        Err(errormsg) => panic!("Error reading {}: {}", DATA_FILE, errormsg),
    };
    // Simpler, "dirtier" version of above, with no error-catching:
    // let data = read_to_string("src/data.txt").unwrap();

    // Convert that long string of data into a series of separate rows
    // that we can iterate over, reusing the variable name.
    let data = data.trim().split('\n');

    // Prepare some "stacks" to hold our columns of integers.
    let mut col_left: Vec<isize> = vec![];
    let mut col_right: Vec<isize> = vec![];

    // Now split each row into a left and right component.
    for line in data {
        // Deal with one row at a time.
        // Break that row up into two components, left & right.
        let left_right: Vec<&str> = line.trim().split_whitespace().collect();

        // Put the left component onto the "left" stack/column,
        col_left.push(
            left_right[0]
                .parse() // converting it to an integer.
                .expect("Error converting text data to integer."),
        );
        // Ditto the right component.
        col_right.push(
            left_right[1]
                .parse()
                .expect("Error converting text data to integer."),
        );
    } // Repeat until all rows are processed.

    // Sort the finished stacks. (unstable is faster, since we don't care to keep
    // the original order of any duplicate numbers.)
    col_left.sort_unstable();
    col_right.sort_unstable();

    // Do the required calculation.
    let mut total = 0;

    for (index, _) in col_left.iter().enumerate() {
        // or, "for index in 0..col_left.len()"
        // Look at each line; get difference between left
        // and right components; add that diff to a running total.
        total += (col_left[index] - col_right[index]).abs();
    } // Repeat until all rows have been processed.

    // Report the running total.
    println!("Total difference = {}", total)
}
