use std::fs;

use day2::{calculate_total_score_1, calculate_total_score_2};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let total_player_score = calculate_total_score_1(&contents);

    println!(
        "Total player score from first decoding strategy: {}",
        total_player_score
    );

    let total_player_score = calculate_total_score_2(&contents);

    println!(
        "Total player score from second decoding strategy: {}",
        total_player_score
    );
}
