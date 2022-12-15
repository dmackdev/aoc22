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
    occupied_positions: &HashMap<i32, Vec<(i32, i32)>>,
    row: i32,
    remove_all_occupied: bool,
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

    let xs_to_remove = match occupied_positions.get(&row) {
        Some(v) => v.to_owned(),
        None => vec![],
    };

    let xs_to_remove = xs_to_remove
        .iter()
        .filter_map(|(x, distance)| {
            if remove_all_occupied {
                return Some(x);
            }

            if *distance == 0 {
                Some(x)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    impossible_xs
        .into_iter()
        .filter(|x| !xs_to_remove.contains(&x))
        .map(|x| Pos::new(x, row))
        .collect::<Vec<_>>()
}

pub fn find_position(occupied_positions: &HashMap<i32, Vec<(i32, i32)>>, max: i32) -> Option<Pos> {
    for row in 0..=max {
        println!("Row {}", row);

        let impossible_positions_for_row =
            get_impossible_positions_for_row(occupied_positions, row, true);

        let impossible_positions_for_row = impossible_positions_for_row
            .into_iter()
            .filter(|pos| 0 <= pos.x && pos.x <= max)
            .collect::<Vec<_>>();

        let num_occupied_positions_in_range = occupied_positions
            .get(&row)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|(x, _)| 0 <= *x && *x <= max)
            .count() as i32;

        if impossible_positions_for_row.len() as i32 + num_occupied_positions_in_range < max + 1 {
            let full_range_set = HashSet::<i32>::from_iter(0..=max);
            let impossible_positions_set =
                HashSet::<i32>::from_iter(impossible_positions_for_row.iter().map(|p| p.x));
            let diff = full_range_set
                .difference(&impossible_positions_set)
                .collect::<HashSet<&i32>>();
            let empty = Vec::new();
            let occupied_set = HashSet::from_iter(
                occupied_positions
                    .get(&row)
                    .unwrap_or(&empty)
                    .iter()
                    .map(|(x, _)| x),
            );
            let diff = diff.difference(&occupied_set).collect::<Vec<_>>();

            let x = diff.first();

            return Some(Pos::new(***x.unwrap(), row));
        }
    }
    None
}

pub fn calculate_tuning_frequency(pos: Pos, max: i32) -> i32 {
    pos.x * max + pos.y
}

fn print_xs(positions: &[Pos]) {
    let mut xs = positions.iter().map(|p| p.x).collect::<Vec<_>>();
    xs.sort();
    println!("{:?}", xs);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn assert_positions(positions: &[Pos], expected_xs: &[i32], expected_y: i32) {
        positions
            .iter()
            .for_each(|pos| assert_eq!(pos.y, expected_y));

        let mut xs = positions.iter().map(|pos| pos.x).collect::<Vec<_>>();
        xs.sort();

        assert_eq!(xs, expected_xs);
    }

    use rstest::rstest;
    #[rstest]
    #[case(9, &[-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,])]
    #[case(10, &[-2, -1, 0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24])]
    #[case(11, &[-3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25])]
    fn example_impossible_positions_for_row(#[case] row: i32, #[case] expected: &[i32]) {
        let occupied_positions =
            parse_occupied_positions(&fs::read_to_string("test_input.txt").unwrap());

        let impossible_positions =
            get_impossible_positions_for_row(&occupied_positions, row, false);

        assert_positions(&impossible_positions, expected, row);
    }

    #[test]
    fn example_find_single_position() {
        let occupied_positions =
            parse_occupied_positions(&fs::read_to_string("test_input.txt").unwrap());

        let position = find_position(&occupied_positions, 20).unwrap();

        assert_eq!(position, Pos::new(14, 11));
        assert_eq!(calculate_tuning_frequency(position, 4000000), 56000011)
    }
}
