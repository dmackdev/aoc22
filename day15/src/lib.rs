use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Pos {
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

pub fn get_impossible_xs_for_row(input: &str, row: i32) -> Vec<i32> {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    let mut sensor_xs = vec![];
    let mut beacon_xs = vec![];
    let mut impossible_xs = HashSet::new();

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

            if sensor_pos.y == row {
                sensor_xs.push(sensor_pos.x);
            }

            if beacon_pos.y == row {
                beacon_xs.push(beacon_pos.x);
            }

            let distance_to_beacon = sensor_pos.distance_to(beacon_pos);

            let distance_to_row = sensor_pos.distance_to(&Pos::new(sensor_pos.x, row));

            if distance_to_row <= distance_to_beacon {
                let row_range = sensor_pos.x - distance_to_beacon + distance_to_row
                    ..=sensor_pos.x + distance_to_beacon - distance_to_row;

                impossible_xs.extend(row_range.into_iter());
            }
        }
    }
    let xs_to_remove = sensor_xs.iter().chain(beacon_xs.iter()).collect::<Vec<_>>();

    impossible_xs
        .into_iter()
        .filter(|x| !xs_to_remove.contains(&x))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let result = get_impossible_xs_for_row(&fs::read_to_string("test_input.txt").unwrap(), 10);

        assert_eq!(result.len(), 26);
    }
}
