use std::{fs, io};

use day1::parse_input;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parsed_input = parse_input(&contents);

    let single_max_total = parsed_input.first().unwrap();
    println!(
        "Total Calories being carried by the Elf carrying the most Calories: {}",
        single_max_total
    );

    let top_three_total: u64 = parsed_input.iter().take(3).sum();
    println!(
        "Total Calories carried by the top three Elves carrying the most Calories: {}",
        top_three_total
    );

    Ok(())
}
