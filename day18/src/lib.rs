use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(',')
            .map(|coord| coord.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}

pub fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|line| line.parse::<Pos>().unwrap())
        .collect()
}

pub fn calculate_exposed_surface_area(cube_positions: Vec<Pos>) -> i32 {
    let mut area = 0;

    for curr_cube_idx in 0..cube_positions.len() {
        area += 6;

        for prev_cube_idx in 0..curr_cube_idx {
            if cube_positions[curr_cube_idx].distance_to(&cube_positions[prev_cube_idx]) == 1 {
                area -= 2;
            }
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn two_adjacent_cubes() {
        let cube_positions = vec![Pos::new(1, 1, 1), Pos::new(2, 1, 1)];
        let exposed_surface_area = calculate_exposed_surface_area(cube_positions);

        assert_eq!(exposed_surface_area, 10);
    }

    #[test]
    fn example() {
        let cube_positions = parse_input(&fs::read_to_string("test_input.txt").unwrap());
        let exposed_surface_area = calculate_exposed_surface_area(cube_positions);

        assert_eq!(exposed_surface_area, 64);
    }
}
