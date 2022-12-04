use std::fs;

use day4::parse_input;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let count = parse_input(&contents);

    println!(
        "The number of assignment pairs where one range fully contains the other: {}",
        count
    );
}
