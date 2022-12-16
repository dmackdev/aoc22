use std::fs;

use day16::{calculate_total_pressure_released, find_path, parse_input};

fn main() {
    let valve_map = parse_input(&fs::read_to_string("input.txt").unwrap());

    let actions = find_path(&valve_map);

    let total_pressure_released = calculate_total_pressure_released(&valve_map, &actions, 30);

    println!("The total pressure released is {}", total_pressure_released);
}
