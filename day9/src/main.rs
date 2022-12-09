use std::fs;

use day9::{parse_input, RopeSimulation};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut sim = RopeSimulation::new();
    let commands = parse_input(&input);

    sim.apply_commands(&commands);

    println!(
        "The number of positions that the tail of the rope visits at least once is: {}",
        sim.visited_tail_positions.len()
    );
}
