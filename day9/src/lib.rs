use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub enum Command {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Command::Up),
            "D" => Ok(Command::Down),
            "L" => Ok(Command::Left),
            "R" => Ok(Command::Right),
            _ => Err(()),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .flat_map(|line| {
            let split: Vec<_> = line.split_whitespace().collect();
            let command = split[0].parse::<Command>().unwrap();
            let num = split[1].parse::<usize>().unwrap();

            vec![command; num]
        })
        .collect()
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point {
    x: i128,
    y: i128,
}

impl Point {
    fn new(x: i128, y: i128) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct RopeSimulation {
    head_pos: Point,
    tail_pos: Point,
    pub visited_tail_positions: HashSet<Point>,
}

impl RopeSimulation {
    pub fn new() -> RopeSimulation {
        let origin = Point::new(0, 0);
        let mut visited_tail_positions = HashSet::new();
        visited_tail_positions.insert(origin);

        RopeSimulation {
            head_pos: origin,
            tail_pos: origin,
            visited_tail_positions,
        }
    }

    pub fn apply_commands(&mut self, commands: &[Command]) {
        for command in commands.iter() {
            self.apply_command(command);
            self.visited_tail_positions.insert(self.tail_pos);
        }
    }

    fn apply_command(&mut self, command: &Command) {
        match command {
            Command::Up => self.head_pos.y += 1,
            Command::Down => self.head_pos.y -= 1,
            Command::Left => self.head_pos.x -= 1,
            Command::Right => self.head_pos.x += 1,
        };

        self.update_tail_pos();
    }

    fn update_tail_pos(&mut self) {
        let diff = self.head_pos - self.tail_pos;

        let (x_diff, y_diff) = (diff.x, diff.y);

        match (x_diff, y_diff) {
            // Same position
            (0, 0) => (),
            // Directly above or below
            (0, y_diff) if y_diff.abs() == 2 => {
                self.tail_pos.y += y_diff.signum();
            }
            // Directly left or right
            (x_diff, 0) if x_diff.abs() == 2 => {
                self.tail_pos.x += x_diff.signum();
            }
            (x_diff, y_diff) if x_diff.abs() > 1 || y_diff.abs() > 1 => {
                self.tail_pos.y += y_diff.signum();
                self.tail_pos.x += x_diff.signum();
            }
            _ => (),
        }
    }
}

impl Default for RopeSimulation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn right_4() {
        let mut sim = RopeSimulation::new();
        let commands = vec![Command::Right; 4];

        sim.apply_commands(&commands);

        assert_eq!(sim.head_pos, Point::new(4, 0));
        assert_eq!(sim.tail_pos, Point::new(3, 0));
    }

    #[test]
    fn diagonal_example_up() {
        let mut sim = RopeSimulation::new();
        sim.head_pos = Point::new(2, 2);
        sim.tail_pos = Point::new(1, 1);
        sim.apply_command(&Command::Up);

        assert_eq!(sim.head_pos, Point::new(2, 3));
        assert_eq!(sim.tail_pos, Point::new(2, 2));
    }

    #[test]
    fn diagonal_example_right() {
        let mut sim = RopeSimulation::new();
        sim.head_pos = Point::new(2, 2);
        sim.tail_pos = Point::new(1, 1);
        sim.apply_command(&Command::Right);

        assert_eq!(sim.head_pos, Point::new(3, 2));
        assert_eq!(sim.tail_pos, Point::new(2, 2));
    }

    #[test]
    fn tail_doesnt_move() {
        let mut sim = RopeSimulation::new();
        sim.head_pos = Point::new(4, 0);
        sim.tail_pos = Point::new(3, 0);
        sim.apply_command(&Command::Up);

        assert_eq!(sim.head_pos, Point::new(4, 1));
        assert_eq!(sim.tail_pos, Point::new(3, 0));
    }

    #[test]
    fn example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let mut sim = RopeSimulation::new();
        let commands = parse_input(&input);

        sim.apply_commands(&commands);

        assert_eq!(sim.head_pos, Point::new(2, 2));
        assert_eq!(sim.tail_pos, Point::new(1, 2));

        let expected_visited_positions = [
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 1),
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(4, 2),
            Point::new(3, 3),
            Point::new(4, 3),
            Point::new(2, 4),
            Point::new(3, 4),
        ];

        assert_eq!(
            sim.visited_tail_positions,
            HashSet::from(expected_visited_positions)
        );
    }
}
