use std::fs;

use day6::find_marker_index;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let marker_index = find_marker_index(&input, 4);

    println!(
        "The first start-of-packet marker index (window size 4) is {}",
        marker_index
    );

    let marker_index = find_marker_index(&input, 14);

    println!(
        "The first start-of-message marker index (window size 14) is {}",
        marker_index
    );
}
