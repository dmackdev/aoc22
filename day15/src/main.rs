use std::fs;

use day15::{get_impossible_positions_for_row, parse_occupied_positions};

fn main() {
    let row = 2000000;
    let occupied_positions = parse_occupied_positions(&fs::read_to_string("input.txt").unwrap());

    let impossible_positions = get_impossible_positions_for_row(occupied_positions, row);

    println!(
        "The number of impossible beacon positions for y={} is: {}",
        row,
        impossible_positions.len()
    )
}
