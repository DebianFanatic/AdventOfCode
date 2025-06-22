mod funcs;
use funcs::*;

const TARGET: &str = "XMAS";

fn main() {
    let grid = get_data_grid("src/data.txt");

    let lastrow = grid.len();
    let lastcolumn = grid[0].len();
    let mut total = 0;

    // Check each cell of the grid for any match in eight directions.
    for row in 0..lastrow {
        for col in 0..lastcolumn {
            let cell = (row, col);
            total += get_num_of_matches(&grid, cell, TARGET);
        }
    }
    println!("Total matches = {}", total);
} // end of main()
