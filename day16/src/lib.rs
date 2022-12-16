use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
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
        let flow_rate = caps.name("rate").unwrap().as_str().parse::<i32>()?;
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

pub fn parse_input(input: &str) -> HashMap<String, Valve> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let valve = line.parse::<Valve>().unwrap();
        acc.insert(valve.name.clone(), valve);
        acc
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let valves = parse_input(&fs::read_to_string("test_input.txt").unwrap());
        println!("{:#?}", valves);
    }
}
