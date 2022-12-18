use std::fs;

use day18::{calculate_exposed_surface_area, parse_input};

fn main() {
    let cube_positions = parse_input(&fs::read_to_string("input.txt").unwrap());
    let exposed_surface_area = calculate_exposed_surface_area(cube_positions);

    println!(
        "The total exposed surface area is: {}",
        exposed_surface_area
    );
}
