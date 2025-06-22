use std::fs::read_to_string;

/// "get_num_of_matches_per_cell" finds all matches in eight directions in
/// a grid of text, and returns the count (0-8).
pub fn get_num_of_matches_per_cell(
    grid: &Vec<Vec<char>>,
    cell: (usize, usize),
    target_string: &str,
) -> usize {
    let row = cell.0;
    let col = cell.1;
    let length_of_target = target_string.len();
    let num_of_cols = grid[row].len();
    let num_of_rows = grid.len();
    let mut num_of_matches = 0;

    // There are eight directions to check. We'll start with East.

    // 1. EAST
    // Check that there's enough chars in the specified direction to hold the target.
    let available_columns = num_of_cols - col;
    if length_of_target <= available_columns {
        // Collect the chars in the specified direction into a String named "search_string",
        // starting at coords (y,x).
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row][col + c]); // <-- This collects chars forward/East.
        }
        // If a match is found, add it to the count.
        if search_string == target_string {
            num_of_matches += 1;
        }
    } //end of EAST

    // 2. SOUTH EAST
    let available_columns = num_of_cols - col;
    let available_rows = num_of_rows - row;
    if length_of_target <= available_columns && length_of_target <= available_rows {
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row + c][col + c]); // <-- This collects chars South-Easterly.
        }
        if search_string == target_string {
            num_of_matches += 1;
        }
    } // end of SOUTH EAST

    // 3. SOUTH
    let available_rows = num_of_rows - row;
    if length_of_target <= available_rows {
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row + c][col]);
        }
        if search_string == target_string {
            num_of_matches += 1;
        }
    } // end of SOUTH

    // 4. SOUTH WEST
    let available_columns = col + 1;
    let available_rows = num_of_rows - row;
    if length_of_target <= available_rows && length_of_target <= available_columns {
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row + c][col - c]); // <-- South Westerly
        }
        if search_string == target_string {
            num_of_matches += 1;
        }
    } // end of SOUTH WEST

    // 5. WEST
    let available_columns = col + 1;
    if length_of_target <= available_columns {
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row][col - c]);
        }
        if search_string == target_string {
            num_of_matches += 1;
        }
    } // end WEST

    // 6. NORTH WEST
    let available_columns = col + 1;
    let available_rows = row + 1;
    if length_of_target <= available_columns && length_of_target <= available_rows {
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row - c][col - c]);
        }
        if search_string == target_string {
            num_of_matches += 1;
        }
    } // end NORTH WEST

    // 7. NORTH
    let available_rows = row + 1;
    if length_of_target <= available_rows {
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row - c][col]);
        }
        if search_string == target_string {
            num_of_matches += 1;
        }
    } // end NORTH

    // 8. NORTH EAST
    let available_columns = num_of_cols - col;
    let available_rows = row + 1;
    if length_of_target <= available_columns && length_of_target <= available_rows {
        let mut search_string: String = String::new();
        for c in 0..length_of_target {
            search_string.push(grid[row - c][col + c]);
        }
        if search_string == target_string {
            num_of_matches += 1;
        }
    } // end NORTH EAST

    num_of_matches
} // end of get_num_of_matches_per_cell()

pub fn get_data_grid(filename: &str) -> Vec<Vec<char>> {
    // Read data file into "data" String.
    let err_msg = format!("Error reading data file \"{}\".", filename);
    let mut data = read_to_string(filename).expect(&err_msg);

    // Trim whitespace from beginning/end of "data".
    data = data.trim().to_string();

    // Split "data" into an iterator over the lines, and collect them into a vector.
    let data: Vec<String> = data.lines().map(|line| line.to_string()).collect();

    // Now convert that Vec<String> to a Vec<Vec<char> grid.
    let grid: Vec<Vec<char>> = data.iter().map(|s| s.chars().collect()).collect();

    grid
} // end of get_data_grid()

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<char>>) {
    let lastrow = grid.len();
    let lastcolumn = grid[0].len();

    println!("    1234567890");
    for row in 0..lastrow {
        print!("{:>2}. ", row + 1);
        for column in 0..lastcolumn {
            print!("{}", grid[row][column]);
        }
        println!();
    }
} // end of print_grid
