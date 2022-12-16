use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub const START_VALVE_NAME: &str = "AA";

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    let remaining_actions =
        vec![Action::Noop; std::cmp::max(mins as i32 - path.len() as i32, 0) as usize];

    path.iter()
        .chain(remaining_actions.iter())
        .take(mins as usize)
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

pub fn find_path(valve_map: &ValveMap) -> Vec<Action> {
    let mut current_valve_name = START_VALVE_NAME.to_string();
    let mut path = vec![];
    let mut valid_valve_names = valve_map
        .values()
        .filter_map(|v| {
            if v.flow_rate > 0 {
                Some(v.name.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    valid_valve_names.sort_by_key(|v_name| valve_map.get(v_name).unwrap().flow_rate);

    while !valid_valve_names.is_empty() {
        let (next_path, _) = valid_valve_names
            .iter()
            .filter_map(|valid_valve| {
                dijkstra(
                    &current_valve_name,
                    |v_name| {
                        valve_map
                            .get(v_name)
                            .unwrap()
                            .tunnels
                            .clone()
                            .into_iter()
                            .zip(std::iter::repeat(1))
                    },
                    |v_name| v_name == valid_valve,
                )
            })
            .max_by_key(|(path, steps)| {
                let cumulative_flow_rate: u32 = path
                    .iter()
                    .skip(1)
                    .map(|v_name| {
                        if valid_valve_names.contains(v_name) {
                            valve_map.get(v_name).unwrap().flow_rate
                        } else {
                            0
                        }
                    })
                    .sum();
                OrderedFloat(cumulative_flow_rate as f64 / *steps as f64)
            })
            .unwrap();

        let next_destination = next_path.last().unwrap().clone();
        valid_valve_names.retain(|v| *v != next_destination);
        path.append(
            &mut next_path[1..next_path.len() - 1]
                .iter()
                .map(|m| Action::MoveTo(m.to_string()))
                .collect(),
        );
        path.push(Action::MoveTo(next_destination.to_string()));
        path.push(Action::Open(next_destination.to_string()));
        current_valve_name = next_destination;
    }
    path
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example_constructed_path() {
        let valve_map = parse_input(&fs::read_to_string("test_input.txt").unwrap());

        let actions = find_path(&valve_map);

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

        assert_eq!(actions, expected_actions);

        let total_pressure_released = calculate_total_pressure_released(&valve_map, &actions, 30);

        assert_eq!(total_pressure_released, 1651);
    }
}
