use std::{cmp::Reverse, collections::HashMap};

use regex::Regex;

pub struct Monkey {
    items: Vec<i128>,
    operation: Box<dyn Fn(i128) -> i128>,
    test: Box<dyn Fn(i128) -> usize>,
    num_inspections: u128,
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    let starting_items_re = Regex::new(r"(\d+)").unwrap();
    let operation_re = Regex::new(r"new = old (\*|\+) (\d+|old)").unwrap();
    let test_divisor_re = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let true_branch_idx_re = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let false_branch_idx_re = Regex::new(r"If false: throw to monkey (\d+)").unwrap();

    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(7)
        .map(|group| {
            let items: Vec<_> = starting_items_re
                .find_iter(group[1])
                .map(|mat| mat.as_str().parse::<i128>().unwrap())
                .collect();

            let operation_re_caps = operation_re.captures(group[2]).unwrap();

            let operation = get_operation_function(
                operation_re_caps.get(1).unwrap().as_str(),
                operation_re_caps.get(2).unwrap().as_str(),
            );

            let divisor = test_divisor_re
                .captures(group[3])
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i128>()
                .unwrap();

            let true_branch_idx = true_branch_idx_re
                .captures(group[4])
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            let false_branch_idx = false_branch_idx_re
                .captures(group[5])
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            let test = get_monkey_test_function(divisor, true_branch_idx, false_branch_idx);

            Monkey {
                items,
                operation,
                test,
                num_inspections: 0,
            }
        })
        .collect()
}

fn get_operation_function(operator: &str, operand: &str) -> Box<dyn Fn(i128) -> i128> {
    let operator = if operator == "*" {
        |x: i128, y: i128| x * y
    } else {
        |x: i128, y: i128| x + y
    };

    if operand == "old" {
        Box::new(move |old: i128| operator(old, old))
    } else {
        let operand = operand.parse::<i128>().unwrap();
        Box::new(move |old: i128| operator(old, operand))
    }
}

fn get_monkey_test_function(
    divisor: i128,
    true_branch_idx: usize,
    false_branch_idx: usize,
) -> Box<dyn Fn(i128) -> usize> {
    let f = move |new: i128| {
        if new % divisor == 0 {
            true_branch_idx
        } else {
            false_branch_idx
        }
    };

    Box::new(f)
}

pub fn process_monkeys(monkeys: &mut Vec<Monkey>, num_rounds: usize) {
    let mut items_to_pass: HashMap<usize, Vec<i128>> = HashMap::new();

    for _ in 0..num_rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            if let Some(mut items_to_update) = items_to_pass.remove(&idx) {
                monkey.items.append(&mut items_to_update);
            }

            for item in monkey.items.iter() {
                let monkey_op = monkey.operation.as_ref();
                let new_item = monkey_op(*item) / 3;

                let monkey_test = monkey.test.as_ref();
                let new_monkey_idx = monkey_test(new_item);

                items_to_pass
                    .entry(new_monkey_idx)
                    .and_modify(|v| v.push(new_item))
                    .or_insert_with(|| vec![new_item]);
            }

            monkey.num_inspections += monkey.items.len() as u128;
            monkey.items.clear();
        }
    }

    for (idx, items) in items_to_pass {
        monkeys[idx].items = items;
    }
}

pub fn calculate_monkey_business(monkeys: &[Monkey]) -> i128 {
    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort_by_key(|i| Reverse(*i));
    (inspections[0] * inspections[1]) as i128
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn example() {
        let input = fs::read_to_string("test_input.txt").unwrap();
        let mut monkeys = parse_input(&input);
        process_monkeys(&mut monkeys, 20);

        assert_eq!(monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[0].num_inspections, 101);

        assert_eq!(monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[1].num_inspections, 95);

        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[2].num_inspections, 7);

        assert_eq!(monkeys[3].items, vec![]);
        assert_eq!(monkeys[3].num_inspections, 105);

        assert_eq!(calculate_monkey_business(&monkeys), 10605);
    }
}
