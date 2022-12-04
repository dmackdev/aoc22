use std::fs;

use day4::{count_fully_contained, count_overlaps};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let count = count_fully_contained(&contents);

    println!(
        "The number of assignment pairs where one range fully contains the other: {}",
        count
    );

    let count = count_overlaps(&contents);

    println!(
        "The number of assignment pairs where one range fully overlaps the other: {}",
        count
    );
}
