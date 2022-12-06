use std::fs;

use day6::find_marker_index;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let marker_index = find_marker_index(&input);

    println!("The first start-of-packet marker index is {}", marker_index);
}
