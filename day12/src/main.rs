use day12::{parse_input, EnhancedGrid};
use std::fs;

fn main() {
    let (grid, start_pos, end_pos) = parse_input(&fs::read_to_string("input.txt").unwrap());

    let (_, cost) = grid.calculate_shortest_path(start_pos, end_pos).unwrap();

    println!("Shortest path length: {}", cost);
}
