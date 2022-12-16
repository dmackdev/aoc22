use lazy_static::lazy_static;
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

pub fn find_best_path(valve_map: &ValveMap, mins: u32) -> (Vec<Action>, u32) {
    let possible_paths = find_paths(valve_map, mins);

    let mut best_path = None;
    let mut total = 0;

    for path in possible_paths {
        let new_total = calculate_total_pressure_released(valve_map, &path, 30);

        if new_total > total {
            best_path = Some(path);
            total = new_total;
        }
    }

    (best_path.unwrap(), total)
}

pub fn find_paths(valve_map: &ValveMap, mins: u32) -> Vec<Vec<Action>> {
    let current_valve_name = START_VALVE_NAME.to_string();
    let mut paths = vec![];
    let valid_valve_names = valve_map
        .values()
        .filter_map(|v| {
            if v.flow_rate > 0 {
                Some(v.name.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    paths.append(&mut find_path_given(
        current_valve_name,
        &valid_valve_names,
        valve_map,
    ));

    let num_iterations = valid_valve_names.len();
    println!("Total iterations: {}", num_iterations);

    for i in 0..num_iterations {
        println!("Iteration {} of {}", i + 1, num_iterations);

        let mut new = paths
            .iter()
            .flat_map(|path| {
                if let Some(Action::Open(last_valve_name)) = path.last() {
                    let curr_valid_valve_names = &valid_valve_names
                        .clone()
                        .into_iter()
                        .filter(|v| {
                            !path
                                .iter()
                                .filter_map(|act| match act {
                                    Action::Open(v_name) => Some(v_name),
                                    _ => None,
                                })
                                .any(|x| x == v)
                        })
                        .collect::<Vec<_>>();

                    let new_paths = find_path_given(
                        last_valve_name.to_string(),
                        curr_valid_valve_names,
                        valve_map,
                    );

                    return new_paths
                        .iter()
                        .filter_map(|new_path| {
                            if path.len() + new_path.len() > mins as usize {
                                None
                            } else {
                                let mut result = path.clone();
                                result.append(&mut new_path.clone());
                                Some(result)
                            }
                        })
                        .collect::<Vec<_>>();
                }
                vec![]
            })
            .collect::<Vec<_>>();

        paths.retain(|p| p.len() - 2 > i); // Remove previous paths of 2 less than i or smaller because we have added minimum two actions on the last iteration - MoveTo and Open
        paths.append(&mut new);
    }
    paths
}

fn find_path_given(
    start: String,
    valid_valve_names: &Vec<String>,
    valve_map: &ValveMap,
) -> Vec<Vec<Action>> {
    let mut paths = vec![];

    for valid_valve_name in valid_valve_names {
        let path = dijkstra(
            &start,
            |v_name| {
                valve_map
                    .get(v_name)
                    .unwrap()
                    .tunnels
                    .clone()
                    .into_iter()
                    .zip(std::iter::repeat(1))
            },
            |v_name| v_name == valid_valve_name,
        );

        if let Some((path, _)) = path {
            let destination = path.last().unwrap().clone();

            let mut actions = path[1..path.len() - 1]
                .iter()
                .map(|v_name| Action::MoveTo(v_name.to_string()))
                .collect::<Vec<_>>();

            actions.push(Action::MoveTo(destination.to_string()));
            actions.push(Action::Open(destination.to_string()));

            paths.push(actions);
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example_constructed_path() {
        let valve_map = parse_input(&fs::read_to_string("test_input.txt").unwrap());

        let (actions, total_pressure_released) = find_best_path(&valve_map, 30);

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
        assert_eq!(total_pressure_released, 1651);
    }
}
