use std::fs::read_to_string;

fn main() {
    println!("Advent of Code");
    println!("Day 1, Part 2");

    // Read the data file as a String.
    const DATA_FILE: &str = "src/data.txt";
    let (mut col_left, col_right) = get_data(DATA_FILE);

    // Sort the left column.
    col_left.sort();

    // Do the required calculation.
    let mut count_of_sims = 0;
    let mut sim_score: Vec<isize> = vec![];

    for val in &col_left {
        for each in &col_right {
            if val == each {
                count_of_sims += 1;
            }
            sim_score.push(val * count_of_sims);
            count_of_sims = 0;
        }
    }

    // Report the result.
    let mut total: isize = 0;
    for each in sim_score {
        total += each;
    }
    println!("Total = {}", total);
} // end of main()

fn get_data(source: &str) -> (Vec<isize>, Vec<isize>) {
    let data = match read_to_string(source) {
        Ok(contents) => contents,
        Err(errormsg) => panic!("Error reading {}: {}", source, errormsg),
    };
    // Convert that String into a vector of &strs.
    let data: Vec<&str> = data.trim().split('\n').collect();

    let mut col_left: Vec<isize> = vec![];
    let mut col_right: Vec<isize> = vec![];

    // Now split each line into a left and right column
    for line in data {
        let left_right: Vec<&str> = line.trim().split_whitespace().collect();
        col_left.push(
            left_right[0]
                .parse()
                .expect("Error converting text data to integer."),
        );
        col_right.push(
            left_right[1]
                .parse()
                .expect("Error converting text data to integer."),
        );
    }
    (col_left, col_right)
} // end of get_data()
