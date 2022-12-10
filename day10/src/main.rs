use std::fs;

use day10::{calculate_signal_strength_sum, parse_input, Machine, CRT};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let commands = parse_input(&input);
    let mut machine = Machine::new();

    let register_values = machine.apply_commands(&commands);

    let sigal_strength_sum = calculate_signal_strength_sum(&register_values);

    println!("The signal strength sum is {}", sigal_strength_sum);

    let mut crt = CRT::new(40, 6);
    crt.draw(&register_values);

    println!("{}", crt.get_display());
}
