use std::fs;

use day15::get_impossible_xs_for_row;

fn main() {
    let row = 2000000;
    let result = get_impossible_xs_for_row(&fs::read_to_string("input.txt").unwrap(), row);

    println!(
        "The number of impossible beacon positions for y={} is: {}",
        row,
        result.len()
    )
}
