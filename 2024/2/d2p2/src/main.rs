mod functs;
use functs::*;

fn main() {
    // Gets a list of report, each report containing
    // a set of levels. These levels, per report,
    // can not be less than one level apart, nor
    // greater than 3 levels apart, and the levels
    // must all consecutively increase or all
    // decrease. Such a report is considered
    // "safe". Tolerance standards allow that
    // one level per report may be discarded,
    // and if that allows the other remaining
    // levels to meet the criteria, that report
    // that otherwise would have been "unsafe"
    // will be considered "safe".

    // Get input data.
    const DATAFILE: &str = "src/input.dat";
    let data: Vec<Vec<usize>> = get_dataset(DATAFILE);

    // Initialize a list of reports that are safe...
    let mut safe_reports_index: Vec<usize> = Vec::new();
    // ... and a list of reports that are unsafe.
    let mut unsafe_reports_index: Vec<usize> = Vec::new();

    // Check each report to see if it's safe.
    for row in 0..data.len() {
        // Just a simpler name here for each row-set
        // of levels ....
        let report = &data[row];

        // We could test the entire set, but since
        // many of them will require testing with
        // one level tossed out, we can simplify this
        // program by just eliminating the entire-set
        // test from the get-go (at some possible
        // expense in execution speed).
        let mut report_safe: bool = false;
        for level in 0..report.len() {
            // Make a copy, so we preserve the original.
            let mut report2: Vec<usize> = report.clone();

            // Remove the current level from copy.
            report2.remove(level);

            // Test the report copy.
            if spread_ok(&report2) && check_direction(&report2) {
                report_safe = true;
                safe_reports_index.push(row);
                set_fg("GREEN");
                //                set_fg("REVERSE");
                println!("Report {} is safe {:?}.", row, report);
                //                set_fg("NOREVERSE");
                set_fg("DEFAULT");
                break; // No need to check this report further; it's safe.
            }
        }

        if !report_safe {
            unsafe_reports_index.push(row);
            set_fg("RED");
            //            set_fg("REVERSE");
            println!("Report {} is unsafe {:?}.", row, report);
            //            set_fg("NOREVERSE");
            set_fg("DEFAULT");
        }
    }

    println!("The count of safe reports = {}.", safe_reports_index.len());

    println!(
        "The count of unsafe reports = {}.",
        unsafe_reports_index.len()
    );
} // main()
