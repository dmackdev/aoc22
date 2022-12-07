use std::fs;

use day7::parse_input;

fn main() {
    let input =
        fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

    parse_input(&input);
}
