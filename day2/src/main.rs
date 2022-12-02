use std::fs;

use day2::calculate_total_score;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let total_player_score = calculate_total_score(&contents);

    println!("Total player score: {}", total_player_score);
}
