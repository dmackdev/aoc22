use std::fs;

use day3::{calculate_priority_sum_of_duplicate_items, calculate_sum_of_priorities_of_badges};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let total_priorities_sum = calculate_priority_sum_of_duplicate_items(&contents);

    println!(
        "Sum of priorities of all duplicate items: {}",
        total_priorities_sum
    );

    let badges_priorities_sum = calculate_sum_of_priorities_of_badges(&contents);

    println!("Sum of priorities of badges: {}", badges_priorities_sum);
}
