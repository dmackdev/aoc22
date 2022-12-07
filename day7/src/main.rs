use std::fs;

use day7::parse_input;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let dirs = parse_input(&input);

    let sum: u128 = dirs.iter().map(|n| n.size()).sum();

    println!("Total size of directories: {}", sum);
}
