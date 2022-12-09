use std::fs;

use day9::{parse_input, RopeSimulation};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let commands = parse_input(&input);

    let mut sim = RopeSimulation::new(2);

    sim.apply_commands(&commands);

    println!(
        "For a 2 point rope,the number of positions that the tail of the rope visits at least once is: {}",
        sim.visited_tail_positions.len()
    );

    let mut sim = RopeSimulation::new(10);

    sim.apply_commands(&commands);

    println!(
        "For a 10 point rope, the number of positions that the tail of the rope visits at least once is: {}",
        sim.visited_tail_positions.len()
    );
}
