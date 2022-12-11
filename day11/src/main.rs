use std::fs;

use day11::{calculate_monkey_business, parse_input, process_monkeys};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut monkeys = parse_input(&input);
    process_monkeys(&mut monkeys, 20);

    let monkey_business = calculate_monkey_business(&monkeys);

    println!("The level of monkey business is: {}", monkey_business);
}
