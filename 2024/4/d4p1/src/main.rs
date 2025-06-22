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

            //     if is_forward_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }
            //     if is_forward_down_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }

            //     if is_down_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }

            //     if is_backward_down_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }

            //     if is_backward_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }

            //     if is_backward_up_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }

            //     if is_up_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }

            //     if is_forward_up_match(&grid, row, col, TARGET) {
            //         total += 1;
            //     }
        }
    }
    println!("Total matches = {}", total);
} // end of main()
