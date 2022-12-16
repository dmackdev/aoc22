use std::fs;

use day16::{find_best_path, parse_input};

fn main() {
    let valve_map = parse_input(&fs::read_to_string("input.txt").unwrap());

    let (_, total_pressure_released) = find_best_path(&valve_map, 30);

    println!("The total pressure released is {}", total_pressure_released);
}
