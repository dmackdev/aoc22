use std::fs;

use day14::{parse_input, GridInit, Pos};

fn main() {
    let rock_paths = parse_input(&fs::read_to_string("input.txt").unwrap());
    let mut grid_init = GridInit::new(rock_paths);
    let count = grid_init.drop_sand(Pos::new(0, 500));

    println!("The number of sand particles dropped is: {}", count);

    println!("{}", grid_init.get_grid_display());
}
