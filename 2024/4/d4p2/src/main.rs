const COMMENTS: &str = r#"
    Do a word search puzzle, counting all occurrences of two
    diagonally-centered-intersecting "MAS" sets, whether it's going
    forward or backwards.

    Advent of Code
    Day 4, Part 2
    Kent West - 23.June.2025
"#;

mod funcs;
use funcs::*;

fn main() {
    println!("{}", COMMENTS);

    // Read input file.
    let grid = get_data_grid("src/data.txt");
    //    print_grid(&grid);

    let lastrow = grid.len() - 1;
    let lastcolumn = grid[0].len() - 1;
    let mut total = 0;

    // Check each cell of the grid (except outer border) for 'A'.
    for row in 1..lastrow {
        for col in 1..lastcolumn {
            if grid[row][col] == 'A' {
                // Collect the two diagonals for comparison.
                let nw = grid[row - 1][col - 1];
                let se = grid[row + 1][col + 1];
                let sw = grid[row + 1][col - 1];
                let ne = grid[row - 1][col + 1];

                // Build NorthWest - SouthEast String.
                let mut nw2se: String = String::from("");
                nw2se.push(nw);
                nw2se.push('A');
                nw2se.push(se);

                // SouthWest to NorthEast String.
                let mut sw2ne: String = String::from("");
                sw2ne.push(sw);
                sw2ne.push('A');
                sw2ne.push(ne);

                // Build NorthEast - SouthWest String.
                let mut ne2sw: String = String::from("");
                ne2sw.push(ne);
                ne2sw.push('A');
                ne2sw.push(sw);

                // Build SouthEast - Northwest String.
                let mut se2nw: String = String::from("");
                se2nw.push(se);
                se2nw.push('A');
                se2nw.push(nw);

                if (se2nw == "SAM" || se2nw == "MAS") && (sw2ne == "SAM" || sw2ne == "MAS") {
                    total += 1;
                }
            }
        }
    }
    println!("Total matches = {}", total);
} // end of main()
