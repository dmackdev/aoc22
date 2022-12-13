use std::fs;

use day13::parse_input;

fn main() {
    let result = parse_input(&fs::read_to_string("input.txt").unwrap());

    println!(
        "The sum of the indices of the packets pairs in the correct order is: {}",
        result
    );
}
