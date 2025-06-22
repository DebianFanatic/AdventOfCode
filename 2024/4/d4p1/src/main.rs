//mod lib;
use d4p1::*;

const TARGET: &str = "XMAS";

fn main() {
    let grid = get_data_grid("src/data.txt");

    let lastrow = grid.len();
    let lastcolumn = grid[0].len();
    let mut total = 0;

    for row in 0..lastrow {
        for col in 0..lastcolumn {
            if is_forward_match(&grid, row, col, TARGET) {
                total += 1;
            }
            if is_forward_down_match(&grid, row, col, TARGET) {
                total += 1;
            }

            if is_down_match(&grid, row, col, TARGET) {
                total += 1;
            }

            if is_backward_down_match(&grid, row, col, TARGET) {
                total += 1;
            }

            if is_backward_match(&grid, row, col, TARGET) {
                total += 1;
            }

            if is_backward_up_match(&grid, row, col, TARGET) {
                total += 1;
            }

            if is_up_match(&grid, row, col, TARGET) {
                total += 1;
            }

            if is_forward_up_match(&grid, row, col, TARGET) {
                total += 1;
            }
        }
    }
    println!("Total matches = {}", total);
} // end of main()
