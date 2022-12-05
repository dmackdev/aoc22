use std::fs;

use day5::{get_message, parse_input, run_commands, CrateMoverModel};

fn main() {
    run(CrateMoverModel::CrateMover9000);
    run(CrateMoverModel::CrateMover9001);
}

fn run(mover_model: CrateMoverModel) {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let (mut stacks, commands) = parse_input(&contents);

    run_commands(&mut stacks, commands, mover_model);

    let message = get_message(&stacks);

    println!("The message for {mover_model:?} is: {message}");
}
