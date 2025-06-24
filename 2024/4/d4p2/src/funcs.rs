use std::fs::read_to_string;

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
