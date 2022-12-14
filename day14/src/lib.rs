use std::{cmp, fmt::Display, ops::Add};

use grid::Grid;
use regex::Regex;

#[derive(Debug, Clone)]
enum Tile {
    Rock,
    Air,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Rock => "#",
            Tile::Air => ".",
            Tile::Sand => "o",
        };
        write!(f, "{}", c)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn normalise(&self, offset: i32) -> Pos {
        Pos {
            row: self.row,
            col: self.col + offset,
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Pos {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

fn get_pos_stats(rock_paths: &[Vec<Pos>]) -> (i32, i32, i32) {
    let flattened_paths = rock_paths.iter().flatten();
    let max_row = flattened_paths
        .clone()
        .max_by_key(|pos| pos.row)
        .unwrap()
        .row;
    let min_col = flattened_paths
        .clone()
        .min_by_key(|pos| pos.col)
        .unwrap()
        .col;
    let max_col = flattened_paths.max_by_key(|pos| pos.col).unwrap().col;

    (max_row, min_col, max_col)
}

#[derive(Debug)]
pub struct GridInit {
    grid: Grid<Tile>,
    offset: i32,
}

impl GridInit {
    pub fn new(rock_paths: Vec<Vec<Pos>>) -> Self {
        let (max_row, min_col, max_col) = get_pos_stats(&rock_paths);

        let offset = -min_col;

        let mut grid = Grid::init(
            max_row as usize + 1,
            (max_col - min_col) as usize + 1,
            Tile::Air,
        );

        grid.init_from_paths(rock_paths, offset);

        Self { grid, offset }
    }

    pub fn get_grid_display(&self) -> String {
        let mut display_string = String::new();

        for row in self
            .grid
            .iter()
            .collect::<Vec<_>>()
            .chunks(self.grid.cols())
        {
            let row_string = row.iter().map(|tile| tile.to_string()).collect::<String>();
            display_string.push_str(&row_string);
            display_string.push('\n');
        }

        display_string
    }

    pub fn drop_sand(&mut self, pos: Pos) -> i32 {
        let pos = pos.normalise(self.offset);
        let mut count = 0;

        loop {
            let mut sand_path = SandPathIterator {
                grid: &self.grid,
                curr_pos: pos,
                did_err: false,
            };

            let mut last_pos = None;

            while !sand_path.did_err {
                let next = sand_path.next();

                if sand_path.did_err {
                    return count;
                }

                match next {
                    Some(pos) => last_pos = Some(pos),
                    None => break,
                }
            }

            match last_pos {
                Some(pos) => {
                    self.grid[pos.row as usize][pos.col as usize] = Tile::Sand;
                    count += 1;
                }
                None => return count,
            };
        }
    }
}

trait EnhancedGrid {
    fn init_from_paths(&mut self, rock_paths: Vec<Vec<Pos>>, offset: i32);
    fn fill_rock_path_row(&mut self, start: &Pos, end: &Pos);
    fn fill_rock_path_col(&mut self, start: &Pos, end: &Pos);
    fn get_from_pos(&self, pos: Pos) -> Option<&Tile>;
}

impl EnhancedGrid for Grid<Tile> {
    fn fill_rock_path_row(&mut self, start: &Pos, end: &Pos) {
        let row_idx = start.row as usize;
        let start_col = cmp::min(start.col, end.col) as usize;
        let end_col = cmp::max(start.col, end.col) as usize;
        for i in start_col..=end_col as usize {
            self[row_idx][i] = Tile::Rock;
        }
    }

    fn fill_rock_path_col(&mut self, start: &Pos, end: &Pos) {
        let col_idx = start.col as usize;
        let start_row = cmp::min(start.row, end.row) as usize;
        let end_row = cmp::max(start.row, end.row) as usize;
        for i in start_row..=end_row as usize {
            self[i][col_idx] = Tile::Rock;
        }
    }

    fn get_from_pos(&self, pos: Pos) -> Option<&Tile> {
        self.get(pos.row as usize, pos.col as usize)
    }

    fn init_from_paths(&mut self, rock_paths: Vec<Vec<Pos>>, offset: i32) {
        for path in rock_paths.iter() {
            for pair in path.windows(2) {
                match (pair[0], pair[1]) {
                    (
                        Pos {
                            row: row_a,
                            col: col_a,
                        },
                        Pos {
                            row: row_b,
                            col: col_b,
                        },
                    ) if row_a == row_b && col_a != col_b => self
                        .fill_rock_path_row(&pair[0].normalise(offset), &pair[1].normalise(offset)),
                    (
                        Pos {
                            row: row_a,
                            col: col_a,
                        },
                        Pos {
                            row: row_b,
                            col: col_b,
                        },
                    ) if col_a == col_b && row_a != row_b => self
                        .fill_rock_path_col(&pair[0].normalise(offset), &pair[1].normalise(offset)),
                    _ => panic!(),
                }
            }
        }
    }
}

struct SandPathIterator<'a> {
    grid: &'a Grid<Tile>,
    curr_pos: Pos,
    did_err: bool,
}

impl<'a> Iterator for SandPathIterator<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr_pos = self.curr_pos + Pos::new(1, 0);

        if self.curr_pos.row >= self.grid.rows() as i32 {
            self.did_err = true;
            return None; // Fell off the grid to the bottom
        }

        let mut next_tile = self.grid.get_from_pos(self.curr_pos);

        match next_tile {
            Some(Tile::Air) => return Some(self.curr_pos),
            Some(_) => (),
            None => {
                self.did_err = true;
                return None;
            } // Fell off the grid
        };

        self.curr_pos = self.curr_pos + Pos::new(0, -1);

        if self.curr_pos.col < 0 {
            self.did_err = true;
            return None;
            // Fell off the grid to the left
        }

        next_tile = self.grid.get_from_pos(self.curr_pos);

        match next_tile {
            Some(Tile::Air) => return Some(self.curr_pos),
            Some(_) => (),
            None => {
                self.did_err = true;
                return None;
            } // Fell off the grid
        };

        self.curr_pos = self.curr_pos + Pos::new(0, 2);

        if self.curr_pos.col >= self.grid.cols() as i32 {
            self.did_err = true;
            return None; // Fell off the grid to the right
        }

        next_tile = self.grid.get_from_pos(self.curr_pos);

        match next_tile {
            Some(Tile::Air) => return Some(self.curr_pos),
            Some(_) => (),
            None => {
                self.did_err = true;
                return None;
            } // Fell off the grid
        };

        // Tile is blocked in all three positions, end iteration
        None
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Pos>> {
    let re = Regex::new(r"(?P<col>\d+),(?P<row>\d+)").unwrap();

    let mut rock_paths = vec![];

    for line in input.lines() {
        let new_rock_paths = re
            .captures_iter(line)
            .map(|caps| {
                Pos::new(
                    caps.name("row").unwrap().as_str().parse().unwrap(),
                    caps.name("col").unwrap().as_str().parse().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        rock_paths.push(new_rock_paths);
    }

    rock_paths
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let rock_paths = parse_input(&fs::read_to_string("test_input.txt").unwrap());
        let mut grid_init = GridInit::new(rock_paths);
        let count = grid_init.drop_sand(Pos::new(0, 500));

        assert_eq!(count, 24);

        let expected_display = "\
..........
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.
";

        assert_eq!(grid_init.get_grid_display(), expected_display);
    }
}
