use std::fs;

use day8::{count_visible, max_scenic_score};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let num_visible_trees = count_visible(&input);

    println!("The number of visible trees is: {}", num_visible_trees);

    let max_scenic_score = max_scenic_score(&input);

    println!("The max scenic score is: {}", max_scenic_score);
}
