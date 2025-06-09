use std::fs::*;

fn main() {
    println!("Advent of Code");
    println!("Day 2, Part 1");
    println!("Kent West - 9.June.2025");

    // Get input data.
    let data: Vec<Vec<usize>> = get_data();

    let mut count_of_safe_records = 0;

    for each_record in data {
        // Check to make sure difference between adjacents <= 3...
        // ... and that all elements increase or decrease consecutively.

        let mut spread_ok = false;
        for datum in 0..each_record.len() - 1 {
            if each_record[datum].abs_diff(each_record[datum + 1]) > 3
                || each_record[datum].abs_diff(each_record[datum + 1]) < 1
            {
                spread_ok = false;
                break;
            } else {
                spread_ok = true;
            }
        }

        let mut dir_down = false;
        for datum in 0..each_record.len() - 1 {
            if each_record[datum] <= each_record[datum + 1] {
                dir_down = false;
                break;
            } else {
                dir_down = true;
            }
        }

        let mut dir_up = false;
        for datum in 0..each_record.len() - 1 {
            if each_record[datum] >= each_record[datum + 1] {
                dir_up = false;
                break;
            } else {
                dir_up = true;
            }
        }

        if (dir_down || dir_up) && (spread_ok) {
            count_of_safe_records += 1;
        }
    }

    println!("The count of safe records = {}.", count_of_safe_records);
} // main()

fn get_data() -> Vec<Vec<usize>> {
    const INPUT_FILE: &str = "src/input.txt";
    let file_contents: String = read_to_string(INPUT_FILE).expect("Error reading input file.");
    let mut file_contents: Vec<&str> = file_contents.split("\n").collect();

    // Reject last empty line from data file.
    file_contents.remove(file_contents.len() - 1);

    // Break each record up into individual &str elements.
    let mut data_strs: Vec<Vec<&str>> = Vec::new();
    for each in file_contents {
        data_strs.push(each.split(" ").collect());
    }

    // Convert the &str elements to usize integers.
    let mut data: Vec<Vec<usize>> = Vec::new();
    for outer in data_strs {
        let mut inner_vec: Vec<usize> = Vec::new();
        for inner in outer {
            inner_vec.push(
                inner
                    .parse::<usize>()
                    .expect("Error converting &str to usize."),
            );
        }
        data.push(inner_vec);
    }
    data
} // fn get_data()
