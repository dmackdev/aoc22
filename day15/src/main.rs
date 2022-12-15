use std::fs;

use day15::{
    calculate_tuning_frequency, find_position, get_impossible_positions_for_row,
    parse_occupied_positions,
};

fn main() {
    let row = 2000000;
    let occupied_positions = parse_occupied_positions(&fs::read_to_string("input.txt").unwrap());

    let impossible_positions = get_impossible_positions_for_row(&occupied_positions, row);

    println!(
        "The number of impossible beacon positions for y={} is: {}",
        row,
        impossible_positions.len()
    );

    let max = 4000000;
    let position = find_position(&occupied_positions, max).unwrap();

    println!("The position is {:?}", position);

    println!(
        "Tuning frequency: {}",
        calculate_tuning_frequency(position, max)
    );
}
