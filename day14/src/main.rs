use std::fs;

use day14::{parse_input, GridWrapper, Pos};

fn main() {
    let rock_paths = parse_input(&fs::read_to_string("input.txt").unwrap());

    let mut grid_wrapper = GridWrapper::new(rock_paths.clone());
    let count = grid_wrapper.drop_sand(Pos::new(0, 500));

    println!("The number of sand particles dropped is: {}", count);

    // println!("{}", grid_wrapper.get_grid_display());

    let mut grid_wrapper = GridWrapper::new_with_floor(rock_paths, 500);
    let count = grid_wrapper.drop_sand(Pos::new(0, 500));

    println!(
        "With an infinite floor, the number of sand particles dropped is: {}",
        count
    );

    // println!("{}", grid_wrapper.get_grid_display());
}
