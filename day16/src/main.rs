use std::fs;

use day16::parse_input;

fn main() {
    let valves = parse_input(&fs::read_to_string("test_input.txt").unwrap());
    println!("{:#?}", valves);
}
