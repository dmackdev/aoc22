pub trait Command {
    fn apply(&self, machine: &mut Machine);
}

#[derive(Debug)]
struct AddCommand {
    value: i64,
}

#[derive(Debug)]
struct NoopCommand;

impl Command for AddCommand {
    fn apply(&self, machine: &mut Machine) {
        machine.register_value += self.value;
    }
}

impl Command for NoopCommand {
    fn apply(&self, _machine: &mut Machine) {}
}

pub fn parse_input(input: &str) -> Vec<Box<dyn Command>> {
    input
        .lines()
        .flat_map(|line| {
            let split_line: Vec<&str> = line.split(' ').collect();
            let result: Vec<Box<dyn Command>> = match split_line[..] {
                ["addx", value] => vec![
                    Box::new(NoopCommand),
                    Box::new(AddCommand {
                        value: value.parse::<i64>().unwrap(),
                    }),
                ],
                ["noop"] => vec![Box::new(NoopCommand)],
                _ => panic!("Unhandled input."),
            };
            result
        })
        .collect()
}

pub fn calculate_signal_strength_sum(register_values: &[i64]) -> i64 {
    let indices: [i64; 6] = [20, 60, 100, 140, 180, 220];
    indices
        .iter()
        .map(|i| i * register_values[*i as usize - 1])
        .sum()
}

pub struct Machine {
    register_value: i64,
}

impl Machine {
    pub fn new() -> Self {
        Machine { register_value: 1 }
    }

    pub fn apply_commands(&mut self, commands: &[Box<dyn Command>]) -> Vec<i64> {
        commands
            .iter()
            .map(|command| {
                let res = self.register_value;
                command.apply(self);
                res
            })
            .collect()
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn small_example_test() {
        let input = "\
noop
addx 3
addx -5";

        let commands = parse_input(input);
        let mut machine = Machine::new();

        let register_values = machine.apply_commands(&commands);

        assert_eq!(machine.register_value, -1);
        assert_eq!(register_values, vec![1, 1, 1, 4, 4]);
    }

    #[test]
    fn example_test() {
        let input = fs::read_to_string("test_input.txt").unwrap();

        let commands = parse_input(&input);
        let mut machine = Machine::new();

        let register_values = machine.apply_commands(&commands);

        assert_eq!(register_values[19], 21);
        assert_eq!(register_values[59], 19);
        assert_eq!(register_values[99], 18);
        assert_eq!(register_values[139], 21);
        assert_eq!(register_values[179], 16);
        assert_eq!(register_values[219], 18);

        let sigal_strength_sum = calculate_signal_strength_sum(&register_values);
        assert_eq!(sigal_strength_sum, 13140)
    }
}
