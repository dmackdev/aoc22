use std::collections::HashMap;

use regex::Regex;

pub fn parse_input(input: &str) -> (Stacks, Vec<MoveCommand>) {
    let lines = input.lines().collect::<Vec<_>>();

    // Split on the blank line between the initial crates configuration and the commands
    let split_lines = lines
        .split(|line| line.trim().is_empty())
        .collect::<Vec<_>>();

    let initial_crates_config = split_lines[0];
    let stacks = parse_initial_crates_config(initial_crates_config);

    let commands = parse_commands(split_lines[1]);

    (stacks, commands)
}

type Stacks = Vec<Vec<char>>;
fn parse_initial_crates_config(initial_crates_config: &[&str]) -> Stacks {
    // Reverse the config so the column numbers are first and we build the stack in the correct order
    let mut iter = initial_crates_config.iter().rev();

    let column_numbers = iter.next().unwrap();

    // Build a map of start index -> column index map
    let start_idx_to_column_idx =
        Regex::new(r"\d")
            .unwrap()
            .find_iter(column_numbers)
            .fold(HashMap::new(), |mut acc, x| {
                acc.insert(x.start(), x.as_str().parse::<usize>().unwrap() - 1);
                acc
            });

    let max_column_idx = start_idx_to_column_idx.values().max().unwrap();
    let mut stacks: Stacks = vec![vec![]; *max_column_idx + 1];

    let re = Regex::new(r"[[:upper:]]").unwrap();
    for l in iter {
        for mat in re.find_iter(l) {
            let start_idx = mat.start();
            let column_idx = start_idx_to_column_idx[&start_idx];
            stacks[column_idx].push(mat.as_str().chars().next().unwrap());
        }
    }

    stacks
}

#[derive(Debug)]
pub struct MoveCommand {
    num_to_move: usize,
    from_col_idx: usize,
    to_col_idx: usize,
}

fn parse_commands(commands: &[&str]) -> Vec<MoveCommand> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    commands
        .iter()
        .map(|c| {
            let caps = re.captures(c).unwrap();
            MoveCommand {
                num_to_move: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from_col_idx: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                to_col_idx: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

pub fn run_commands(stacks: &mut Stacks, commands: Vec<MoveCommand>) {
    for command in commands.iter() {
        let move_from_stack = &mut stacks[command.from_col_idx];
        let mut to_move = move_from_stack
            .drain((move_from_stack.len() - command.num_to_move)..)
            .rev()
            .collect::<Vec<_>>();

        let move_to_stack = &mut stacks[command.to_col_idx];
        move_to_stack.append(&mut to_move);
    }
}

pub fn get_message(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn initial_stacks_example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let (stacks, _) = parse_input(&input);
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(stacks, expected);
    }

    #[test]
    fn first_command_only_example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let (mut stacks, commands) = parse_input(&input);

        run_commands(&mut stacks, commands.into_iter().take(1).collect());

        let expected = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        assert_eq!(stacks, expected);
    }

    #[test]
    fn all_commands_example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let (mut stacks, commands) = parse_input(&input);

        run_commands(&mut stacks, commands);

        let expected = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];
        assert_eq!(stacks, expected);
    }

    #[test]
    fn get_message_example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let (mut stacks, commands) = parse_input(&input);

        run_commands(&mut stacks, commands);

        let message = get_message(&stacks);

        let expected = String::from("CMZ");

        assert_eq!(message, expected);
    }
}
