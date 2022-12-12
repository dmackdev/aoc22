use grid::*;
use pathfinding::prelude::dijkstra;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct GridInit {
    grid_vec: Vec<u32>,
    cols: Option<usize>,
    start_pos: Option<Pos>,
    end_pos: Option<Pos>,
}

impl GridInit {
    fn new() -> Self {
        GridInit {
            grid_vec: Vec::new(),
            cols: None,
            start_pos: None,
            end_pos: None,
        }
    }
}

impl From<GridInit> for (Grid<u32>, Pos, Pos) {
    fn from(val: GridInit) -> Self {
        (
            Grid::from_vec(val.grid_vec, val.cols.unwrap()),
            val.start_pos.unwrap(),
            val.end_pos.unwrap(),
        )
    }
}

pub trait EnhancedGrid<T> {
    fn get_neighbours(&self, pos: Pos) -> Vec<Pos>;
    fn get_from_pos(&self, pos: Pos) -> T;
    fn calculate_shortest_path(&self, start_pos: Pos, end_pos: Pos) -> Option<(Vec<Pos>, i32)>;
}

impl EnhancedGrid<u32> for Grid<u32> {
    fn get_neighbours(&self, pos: Pos) -> Vec<Pos> {
        let height = self.get_from_pos(pos);

        let mut neighbours = vec![];

        if 0 < pos.col {
            let new_pos = Pos::new(pos.row, pos.col - 1);
            update_neighbours(self, height, new_pos, &mut neighbours);
        }

        if 0 < pos.row {
            let new_pos = Pos::new(pos.row - 1, pos.col);
            update_neighbours(self, height, new_pos, &mut neighbours);
        }

        if pos.col < self.cols() - 1 {
            let new_pos = Pos::new(pos.row, pos.col + 1);
            update_neighbours(self, height, new_pos, &mut neighbours);
        }

        if pos.row < self.rows() - 1 {
            let new_pos = Pos::new(pos.row + 1, pos.col);
            update_neighbours(self, height, new_pos, &mut neighbours);
        }

        neighbours
    }

    fn get_from_pos(&self, pos: Pos) -> u32 {
        *self.get(pos.row, pos.col).unwrap()
    }

    fn calculate_shortest_path(&self, start_pos: Pos, end_pos: Pos) -> Option<(Vec<Pos>, i32)> {
        dijkstra(
            &start_pos,
            |&p| self.get_neighbours(p).into_iter().map(|p| (p, 1)),
            |&p| p == end_pos,
        )
    }
}

pub fn parse_input(input: &str) -> (Grid<u32>, Pos, Pos) {
    input
        .lines()
        .enumerate()
        .fold(GridInit::new(), |mut grid_init, (row, line)| {
            let (mut row_vec, start_pos, end_pos) =
                line.chars()
                    .enumerate()
                    .fold((vec![], None, None), |mut acc, (col, c)| {
                        match c {
                            'S' => {
                                acc.0.push(to_height('a'));
                                acc.1 = Some(Pos::new(row, col));
                            }
                            'E' => {
                                acc.0.push(to_height('z'));
                                acc.2 = Some(Pos::new(row, col));
                            }
                            _ => acc.0.push(to_height(c)),
                        };

                        acc
                    });

            if grid_init.cols.is_none() {
                grid_init.cols = Some(row_vec.len());
            }

            grid_init.grid_vec.append(&mut row_vec);

            if grid_init.start_pos.is_none() {
                grid_init.start_pos = start_pos;
            }

            if grid_init.end_pos.is_none() {
                grid_init.end_pos = end_pos;
            }

            grid_init
        })
        .into()
}

fn to_height(c: char) -> u32 {
    c as u32 - 'a' as u32
}

fn update_neighbours(grid: &Grid<u32>, height: u32, new_pos: Pos, neighbours: &mut Vec<Pos>) {
    let neighbour_height = grid.get_from_pos(new_pos);
    if height + 1 == neighbour_height || neighbour_height <= height {
        neighbours.push(new_pos);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let (grid, start_pos, end_pos) =
            parse_input(&fs::read_to_string("test_input.txt").unwrap());

        let (_, cost) = grid.calculate_shortest_path(start_pos, end_pos).unwrap();

        assert_eq!(cost, 31);
    }
}
