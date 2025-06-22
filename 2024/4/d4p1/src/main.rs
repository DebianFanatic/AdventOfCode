const COMMENTS: &str = r#"
    Do a word search puzzle, finding all occurrences of the
    word "XMAS", whether it's going up, down, forward,
    backwards, or any of the diagonals - eight different
    directions to check.

    Advent of Code
    Day 4, Part 1
    Kent West - 21.June.2025
"#;

mod funcs;
use funcs::*;

const TARGET: &str = "XMAS";

fn main() {
    println!("{}", COMMENTS);

    // Read input file.
    let grid = get_data_grid("src/data.txt");

    let lastrow = grid.len();
    let lastcolumn = grid[0].len();
    let mut total = 0;

    // Check each cell of the grid for any match in eight directions.
    for row in 0..lastrow {
        for col in 0..lastcolumn {
            let cell = (row, col);
            total += get_num_of_matches_per_cell(&grid, cell, TARGET);
        }
    }
    println!("Total matches = {}", total);
} // end of main()
