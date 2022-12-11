use std::fs;

use day11::{
    calculate_monkey_business, parse_input, process_monkeys, Part1Strategy, Part2Strategy, Strategy,
};

fn main() {
    run(20, Part1Strategy);
    run(10000, Part2Strategy);
}

fn run(num_rounds: usize, strategy: impl Strategy + Copy) {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut monkeys = parse_input(&input);
    process_monkeys(&mut monkeys, num_rounds, strategy);

    let monkey_business = calculate_monkey_business(&monkeys);

    println!(
        "{}: The level of monkey business is: {}",
        strategy.name(),
        monkey_business
    );
}
