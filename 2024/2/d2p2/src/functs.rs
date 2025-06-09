use std::fs::*;

pub fn set_fg(color: &str) {
    // "\x1b[38;2<r>;<g>;<b>m" sets FG color to specified RGB colors.
    // "\x1b[48;2<r>;<g>;<b>m" sets BG color to specified RGB colors.
    match color.to_uppercase().as_str() {
        "RED" => print!("\x1b[31m"),
        "GREEN" => print!("\x1b[32m"),
        "YELLOW" => print!("\x1b[38;2;255;255;0m"),
        "REVERSE" => print!("\x1b[7m"),
        "NOREVERSE" => print!("\x1b[27m"),
        "RESET" => print!("\033[39m\\033[49m"),
        "DEFAULT" => print!("\x1b[39m"),
        _ => println!("Error setting color."),
    }
} //fn set_fg()

pub fn get_dataset(input_file: &str) -> Vec<Vec<usize>> {
    // 'data' looks like the following (each 'row' is a record). This function
    // reads the text file, and converts the rows to a vector of rows, and
    // converts each row-vector of integers into an inner vector of integers,
    // to make a vector consisting of multiple vectors of integers -> Vec<Vec<integer>>
    //
    //         column: 0   1   2   3   4   5   6   7   8
    // row 0 ->     {  3   4   5  23  21   6   9           } <- '4' = data[0][1]
    // row 1 ->     { 44  18   4  22  87 365   2           } <- '22' = data[1][3]
    //  .
    //  .
    //  .

    // Read the file as one long string.
    let file_contents: String = read_to_string(input_file).expect("Error reading input file.");

    // Convert the String to a vector of &str rows holding text-numerals.
    let data_vec: Vec<&str> = file_contents.lines().collect();

    // Split each row of whitespace-separated text-numerals to form a vector of usize integers.
    // Push each of these vectors onto a new vector, to form a vector of usize-holding vectors.
    let mut numbers: Vec<Vec<usize>> = Vec::new();
    for row in data_vec {
        let record: Vec<usize> = row
            .split_whitespace()
            .map(|s| s.parse().expect("Error parsing the numbers."))
            .collect();
        numbers.push(record);
    }

    // Return the dataset.
    numbers
} // get_dataset()

pub fn spread_ok(record: &Vec<usize>) -> bool {
    let mut result: bool = true;
    for col in 0..record.len() - 1 {
        if record[col].abs_diff(record[col + 1]) > 3 || record[col].abs_diff(record[col + 1]) < 1 {
            result = false;
            break;
        }
    }
    result
} // fn spread_ok()

fn descending(record: &Vec<usize>) -> bool {
    // Returns:
    // 'true' if consistent downward trend.
    for col in 0..record.len() - 1 {
        if record[col] <= record[col + 1] {
            return false;
        }
    }
    true
} // fn descend()

fn ascending(record: &Vec<usize>) -> bool {
    // Returns:
    // 'true' if consistent upward trend.
    for col in 0..record.len() - 1 {
        if record[col] >= record[col + 1] {
            return false;
        }
    }
    true
} // fn ascend()

pub fn check_direction(record: &Vec<usize>) -> bool {
    if ascending(record) || descending(record) {
        true
    } else {
        false
    }
}
