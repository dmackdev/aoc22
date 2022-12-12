use day12::{parse_input, EnhancedGrid};
use std::fs;

fn main() {
    let (grid, start_pos, end_pos, lowest_positions) =
        parse_input(&fs::read_to_string("input.txt").unwrap());

    let (_, cost) = grid.calculate_shortest_path(start_pos, end_pos).unwrap();

    println!("Shortest path length: {}", cost);

    let (_, shortest_path_cost_from_lowest_positions) =
        grid.find_shortest_path(lowest_positions, end_pos).unwrap();

    println!(
        "Shortest path out of all lowest positions: {}",
        shortest_path_cost_from_lowest_positions
    );
}
