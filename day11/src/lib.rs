use std::{cmp::Reverse, collections::HashMap};

use regex::Regex;

pub struct Monkey {
    items: Vec<Expr>,
    operation: OperationFunction,
    test: TestFunction,
    num_inspections: u128,
}

impl Monkey {
    fn evaluate_items(&self) -> Vec<i128> {
        self.items.iter().map(|i| i.evaluate()).collect()
    }
}

enum OperationFunction {
    Add(i128),
    MulBy(i128),
    Square,
}

impl OperationFunction {
    fn apply(&self, old: Expr) -> Expr {
        match self {
            OperationFunction::Add(val) => Expr::Add(Box::new(old), Box::new(Expr::Val(*val))),
            OperationFunction::MulBy(val) => Expr::Mul(Box::new(old), Box::new(Expr::Val(*val))),
            OperationFunction::Square => Expr::Mul(Box::new(old.clone()), Box::new(old)),
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Val(i128),
}

impl Expr {
    fn modulo(&self, divisor: i128) -> i128 {
        match self {
            Expr::Add(a, b) => (a.modulo(divisor) + b.modulo(divisor)) % divisor,
            Expr::Mul(a, b) => (a.modulo(divisor) * b.modulo(divisor)) % divisor,
            Expr::Val(v) => v % divisor,
        }
    }

    fn evaluate(&self) -> i128 {
        match self {
            Expr::Add(a, b) => a.evaluate() + b.evaluate(),
            Expr::Mul(a, b) => a.evaluate() * b.evaluate(),
            Expr::Val(v) => *v,
        }
    }
}

struct TestFunction {
    divisor: i128,
    true_branch_idx: usize,
    false_branch_idx: usize,
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
                .map(|mat| Expr::Val(mat.as_str().parse::<i128>().unwrap()))
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

            let test = TestFunction {
                divisor,
                true_branch_idx,
                false_branch_idx,
            };

            Monkey {
                items,
                operation,
                test,
                num_inspections: 0,
            }
        })
        .collect()
}

fn get_operation_function(operator: &str, operand: &str) -> OperationFunction {
    match [operator, operand] {
        ["*", "old"] => OperationFunction::Square,
        ["*", val] => OperationFunction::MulBy(val.parse::<i128>().unwrap()),
        ["+", val] => OperationFunction::Add(val.parse::<i128>().unwrap()),
        _ => panic!(),
    }
}

pub trait Strategy {
    fn calculate_new_item(&self, monkey: &Monkey, item: Expr) -> Expr;
}

pub struct Part1Strategy;

impl Strategy for Part1Strategy {
    fn calculate_new_item(&self, monkey: &Monkey, item: Expr) -> Expr {
        Expr::Val(monkey.operation.apply(item).evaluate() / 3)
    }
}

pub struct Part2Strategy;

impl Strategy for Part2Strategy {
    fn calculate_new_item(&self, monkey: &Monkey, item: Expr) -> Expr {
        monkey.operation.apply(item)
    }
}

pub fn process_monkeys(monkeys: &mut Vec<Monkey>, num_rounds: usize, strategy: impl Strategy) {
    let mut items_to_pass: HashMap<usize, Vec<Expr>> = HashMap::new();

    for _ in 0..num_rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            if let Some(mut items_to_update) = items_to_pass.remove(&idx) {
                monkey.items.append(&mut items_to_update);
            }

            for item in monkey.items.iter() {
                let new_item = strategy.calculate_new_item(monkey, item.clone());

                let new_monkey_idx = if new_item.modulo(monkey.test.divisor) == 0 {
                    monkey.test.true_branch_idx
                } else {
                    monkey.test.false_branch_idx
                };

                items_to_pass
                    .entry(new_monkey_idx)
                    .and_modify(|v| v.push(new_item.clone()))
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
        process_monkeys(&mut monkeys, 20, Part1Strategy);

        assert_eq!(monkeys[0].evaluate_items(), vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[0].num_inspections, 101);

        assert_eq!(monkeys[1].evaluate_items(), vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[1].num_inspections, 95);

        assert_eq!(monkeys[2].evaluate_items(), vec![]);
        assert_eq!(monkeys[2].num_inspections, 7);

        assert_eq!(monkeys[3].evaluate_items(), vec![]);
        assert_eq!(monkeys[3].num_inspections, 105);

        assert_eq!(calculate_monkey_business(&monkeys), 10605);
    }

    #[test]
    #[ignore]
    fn example_part_2() {
        let input = fs::read_to_string("test_input.txt").unwrap();
        let mut monkeys = parse_input(&input);
        process_monkeys(&mut monkeys, 10000, Part2Strategy);

        assert_eq!(calculate_monkey_business(&monkeys), 2713310158);
    }
}
