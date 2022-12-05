use std::fs;

use day5::{get_message, parse_input, run_commands};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let (mut stacks, commands) = parse_input(&contents);

    run_commands(&mut stacks, commands);

    let message = get_message(&stacks);

    println!("The message is: {message}",);
}
