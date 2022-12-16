use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Action {
    MoveTo(String),
    Open(String),
    Noop,
}

impl FromStr for Valve {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve (?P<name>[[:upper:]]+) has flow rate=(?P<rate>\d+); tunnel(s?) lead(s?) to valve(s?) (?P<tunnels>.+)$")
                    .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let name = caps.name("name").unwrap().as_str().to_string();
        let flow_rate = caps.name("rate").unwrap().as_str().parse::<u32>()?;
        let tunnels = caps
            .name("tunnels")
            .unwrap()
            .as_str()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        Ok(Self {
            name,
            flow_rate,
            tunnels,
        })
    }
}

type ValveMap = HashMap<String, Valve>;

pub fn parse_input(input: &str) -> ValveMap {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let valve = line.parse::<Valve>().unwrap();
        acc.insert(valve.name.clone(), valve);
        acc
    })
}

pub fn calculate_total_pressure_released(
    valve_map: &ValveMap,
    path: &Vec<Action>,
    mins: u32,
) -> u32 {
    let remaining_actions = vec![Action::Noop; mins as usize - path.len()];

    path.iter()
        .chain(remaining_actions.iter())
        .fold(
            (0, 0),
            |(current_open_pressure, accumulated_pressure), action| match action {
                Action::Open(valve_name) => (
                    current_open_pressure + valve_map.get(valve_name).unwrap().flow_rate,
                    accumulated_pressure + current_open_pressure,
                ),
                _ => (
                    current_open_pressure,
                    accumulated_pressure + current_open_pressure,
                ),
            },
        )
        .1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example_constructed_path() {
        let valve_map = parse_input(&fs::read_to_string("test_input.txt").unwrap());

        let expected_actions = [
            Action::MoveTo(String::from("DD")),
            Action::Open(String::from("DD")),
            Action::MoveTo(String::from("CC")),
            Action::MoveTo(String::from("BB")),
            Action::Open(String::from("BB")),
            Action::MoveTo(String::from("AA")),
            Action::MoveTo(String::from("II")),
            Action::MoveTo(String::from("JJ")),
            Action::Open(String::from("JJ")),
            Action::MoveTo(String::from("II")),
            Action::MoveTo(String::from("AA")),
            Action::MoveTo(String::from("DD")),
            Action::MoveTo(String::from("EE")),
            Action::MoveTo(String::from("FF")),
            Action::MoveTo(String::from("GG")),
            Action::MoveTo(String::from("HH")),
            Action::Open(String::from("HH")),
            Action::MoveTo(String::from("GG")),
            Action::MoveTo(String::from("FF")),
            Action::MoveTo(String::from("EE")),
            Action::Open(String::from("EE")),
            Action::MoveTo(String::from("DD")),
            Action::MoveTo(String::from("CC")),
            Action::Open(String::from("CC")),
        ]
        .to_vec();

        let total_pressure_released =
            calculate_total_pressure_released(&valve_map, &expected_actions, 30);

        assert_eq!(total_pressure_released, 1651);
    }
}
