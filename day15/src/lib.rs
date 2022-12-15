use regex::Regex;
use std::{
    cmp,
    collections::{HashMap, HashSet},
};

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
        .filter_map(|(x, distance)| if *distance == 0 { Some(x) } else { None })
        .collect::<Vec<_>>();

    impossible_xs
        .into_iter()
        .filter(|x| !xs_to_remove.contains(&x))
        .map(|x| Pos::new(x, row))
        .collect::<Vec<_>>()
}

pub fn find_position(occupied_positions: &HashMap<i32, Vec<(i32, i32)>>, max: i32) -> Option<Pos> {
    let mut out_of_range_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (row, occupants) in occupied_positions.iter() {
        for (x, distance_to_beacon) in occupants {
            if *distance_to_beacon == 0 {
                // This is a beacon, skip
            } else {
                let mut count = 0;
                let mut passed_row = false;
                for row_to_mark in row - distance_to_beacon - 1..=row + distance_to_beacon + 1 {
                    let start = cmp::min(x - count, x + count);
                    let end = cmp::max(x - count, x + count);

                    for x_to_mark in [start, end] {
                        if 0 <= row_to_mark
                            && row_to_mark <= max
                            && 0 <= x_to_mark
                            && x_to_mark <= max
                        {
                            out_of_range_map
                                .entry(row_to_mark)
                                .and_modify(|v| {
                                    v.insert(x_to_mark);
                                })
                                .or_insert_with(|| {
                                    let mut set = HashSet::new();
                                    set.insert(x_to_mark);
                                    set
                                });
                        }
                    }

                    if count > *distance_to_beacon {
                        passed_row = true;
                    }

                    if passed_row {
                        count -= 1;
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }
    // println!("{:#?}", out_of_range_map);

    for (row, out_of_range_xs) in out_of_range_map.iter() {
        for x in out_of_range_xs {
            let pos = Pos::new(*x, *row);
            // Is pos out of range for all sensors?
            for (sensor_row, sensor_xs) in occupied_positions {
                if sensor_xs.iter().all(|(sensor_x, sensor_distance)| {
                    if *sensor_distance == 0 {
                        return true;
                    }

                    let sensor_pos = Pos::new(*sensor_x, *sensor_row);

                    sensor_pos.distance_to(&pos) > *sensor_distance
                }) {
                    return Some(pos);
                }
            }
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

        let impossible_positions = get_impossible_positions_for_row(&occupied_positions, row);

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
