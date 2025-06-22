use std::fs::read_to_string;

pub fn num_of_matches(grid: &Vec<Vec<char>>, row: usize, col: usize, target_string: &str) -> usize {
    // There are eight directions to check. We'll start with forward'
} // end of num_of_matches()

pub fn is_forward_match(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    target_string: &str,
) -> bool {
    let length_of_target = target_string.len();
    let num_of_cols = grid[row].len();
    let num_of_rows = grid.len();

    // Check that there's enough chars in the specified direction to hold the target.
    let distance_to_col_edge = num_of_cols - col;
    if length_of_target > distance_to_col_edge {
        return false;
    } // If not, there will be no match this time 'round.

    // Collect the chars in the specified direction into a String
    let mut search_string: String = String::new();
    for c in 0..length_of_target {
        search_string.push(grid[row + c][col + c]);
    }

    // Check if the search string matches the target string.
    if search_string == target_string {
        return true;
    }
    false
} // end of is_forward_match()

pub fn is_forward_down_match(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    target_string: &str,
) -> bool {
    let length_of_target = target_string.len();
    let num_of_cols = grid[row].len();
    let num_of_rows = grid.len();

    // Check that there's enough chars in the specified direction to hold the target.
    if (num_of_rows as isize - length_of_target as isize) < row as isize
        || (num_of_cols as isize - length_of_target as isize) < col as isize
    {
        return false;
    } // If not, there will be no match this time 'round.

    let mut search_string: String = String::new();
    for c in 0..length_of_target {
        search_string.push(grid[row + c][col + c]);
    }

    if search_string == target_string {
        return true;
    }
    false
} // end of is_forward_down_match()

pub fn is_down_match(grid: &Vec<Vec<char>>, row: usize, col: usize, target: &str) -> bool {
    let length_of_target = target.len();
    let num_of_cols = grid[row].len();
    let num_of_rows = grid.len();

    if (num_of_rows as isize - length_of_target as isize) < row as isize {
        return false;
    }
    if (num_of_cols as isize - length_of_target as isize) < col as isize {
        return false;
    }

    let mut search_string: String = String::new();
    for c in 0..length_of_target {
        search_string.push(grid[row + c][col]);
    }
    // println!(
    //     "The downward string from ({},{}) to ({},{}) is {}",
    //     row,
    //     col,
    //     row + length_of_target,
    //     col,
    //     search_string
    // );
    if search_string == target {
        return true;
    }
    false
    //    let search_field_length = length_of_row - col;
} // end of is_down_match()

pub fn is_backward_down_match(grid: &Vec<Vec<char>>, row: usize, col: usize, target: &str) -> bool {
    let length_of_target = target.len();
    let num_of_rows = grid.len();

    if row + 1 > num_of_rows - length_of_target {
        return false;
    }
    if col + 1 < length_of_target {
        return false;
    }

    let mut search_string: String = String::new();
    for c in 0..length_of_target {
        search_string.push(grid[row + c][col - c]);
    }

    if search_string == target {
        return true;
    }
    false
} // end of is_backward_down_match()

pub fn is_backward_match(grid: &Vec<Vec<char>>, row: usize, col: usize, target: &str) -> bool {
    let length_of_target = target.len();
    if (col as isize - length_of_target as isize) < 0 {
        return false;
    }
    let col: usize = col - length_of_target;
    let target: String = target.chars().rev().collect();
    let length_of_row = grid[row].len();
    let search_field_length = length_of_row - col;

    if search_field_length < length_of_target {
        return false;
    }
    let search_field: &String = &grid[row][col..(col + length_of_target)].iter().collect();

    // println!("Length of target: {}", length_of_target);
    // println!("Length of row: {}", length_of_row);
    // println!("length of search field: {}", search_field_length);
    // println!("Column: {}", col);
    // println!("Search Field = {}", search_field);
    if search_field == target.as_str() {
        return true;
    }
    false
} // end of is_backward_match()

pub fn is_backward_up_match(grid: &Vec<Vec<char>>, row: usize, col: usize, target: &str) -> bool {
    let length_of_target = target.len();
    let num_of_cols = grid[row].len();
    let num_of_rows = grid.len();

    if (num_of_rows as isize - length_of_target as isize) < row as isize {
        return false;
    }
    if (num_of_cols as isize - length_of_target as isize) < col as isize {
        return false;
    }

    let mut search_string: String = String::new();
    for c in 0..length_of_target {
        search_string.push(grid[row - c][col - c]);
    }

    if search_string == target {
        return true;
    }
    false
} // end of is_backward_up_match()

pub fn is_up_match(grid: &Vec<Vec<char>>, row: usize, col: usize, target: &str) -> bool {
    let length_of_target = target.len();
    let num_of_cols = grid[row].len();
    let num_of_rows = grid.len();

    if (num_of_rows as isize - length_of_target as isize) < row as isize {
        return false;
    }
    if (num_of_cols as isize - length_of_target as isize) < col as isize {
        return false;
    }

    let mut search_string: String = String::new();
    for c in 0..length_of_target {
        search_string.push(grid[row + c][col]);
    }

    if search_string == target {
        return true;
    }
    false
} // end of is_up_match()

pub fn is_forward_up_match(grid: &Vec<Vec<char>>, row: usize, col: usize, target: &str) -> bool {
    let length_of_target = target.len();
    let num_of_cols = grid[row].len();
    let num_of_rows = grid.len();

    if (num_of_rows as isize - length_of_target as isize) < row as isize {
        return false;
    }
    if (num_of_cols as isize - length_of_target as isize) < col as isize {
        return false;
    }

    let mut search_string: String = String::new();
    for c in 0..length_of_target {
        search_string.push(grid[row - c][col + c]);
    }

    if search_string == target {
        return true;
    }
    false
} // end of is_forward_up_match()

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
} // end of print_grid()
