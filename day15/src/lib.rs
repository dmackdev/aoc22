use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn parse_occupied_positions(input: &str) -> HashMap<i32, Vec<(i32, i32)>> {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    let mut occupied_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for line in input.lines() {
        for pair in re
            .captures_iter(line)
            .map(|caps| {
                Pos::new(
                    caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>()
            .chunks(2)
        {
            let sensor_pos = &pair[0];
            let beacon_pos = &pair[1];

            let distance_to_beacon = sensor_pos.distance_to(beacon_pos);

            occupied_map
                .entry(sensor_pos.y)
                .and_modify(|row| row.push((sensor_pos.x, distance_to_beacon)))
                .or_insert_with(|| vec![(sensor_pos.x, distance_to_beacon)]);

            occupied_map
                .entry(beacon_pos.y)
                .and_modify(|row| row.push((beacon_pos.x, 0)))
                .or_insert_with(|| vec![(beacon_pos.x, 0)]);
        }
    }

    occupied_map
}

pub fn get_impossible_positions_for_row(
    occupied_positions: HashMap<i32, Vec<(i32, i32)>>,
    row: i32,
) -> Vec<Pos> {
    let mut impossible_xs = HashSet::new();

    for (y, occupants) in occupied_positions.iter() {
        for (x, distance_to_beacon) in occupants {
            let distance_to_row = Pos::new(*x, *y).distance_to(&Pos::new(*x, row));

            if distance_to_row <= *distance_to_beacon {
                let row_range = x - distance_to_beacon + distance_to_row
                    ..=x + distance_to_beacon - distance_to_row;
                impossible_xs.extend(row_range.into_iter());
            }
        }
    }

    let xs_to_remove = occupied_positions
        .get(&row)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|p| p.0)
        .collect::<Vec<_>>();

    impossible_xs
        .into_iter()
        .filter(|x| !xs_to_remove.contains(x))
        .map(|x| Pos::new(x, row))
        .collect::<Vec<_>>()
}

fn find_position(impossible_positions: Vec<Pos>, max: i32) -> Option<Pos> {
    for row in 0..=max {}
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let occupied_positions =
            parse_occupied_positions(&fs::read_to_string("test_input.txt").unwrap());

        let impossible_positions = get_impossible_positions_for_row(occupied_positions, 10);

        assert_eq!(impossible_positions.len(), 26);

        // assert_eq!(find_position(impossible_positions), Some(Pos::new(14, 11)));
    }
}
