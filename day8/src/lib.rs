use grid::Grid;

pub mod grid;

pub fn count_visible(input: &str) -> usize {
    let grid = input.parse::<Grid<u8>>().unwrap();

    grid.iter()
        .enumerate()
        .filter(|(idx, _)| grid.is_visible(*idx))
        .count()
}

pub fn max_scenic_score(input: &str) -> u32 {
    let grid = input.parse::<Grid<u8>>().unwrap();

    grid.iter()
        .enumerate()
        .map(|(idx, _)| grid.scenic_score(idx))
        .max()
        .unwrap()
}

trait Visible {
    fn is_visible(&self, idx: usize) -> bool;
    fn scenic_score(&self, idx: usize) -> u32;
}

impl<'a> Visible for Grid<'a, u8> {
    fn is_visible(&self, idx: usize) -> bool {
        let (row_idx, col_idx) = self.to_row_col(idx);

        if row_idx == 0
            || col_idx == 0
            || row_idx == self.num_rows() - 1
            || col_idx == self.num_columns() - 1
        {
            return true;
        }

        let height = self.items[row_idx][col_idx];

        let row: Vec<_> = self.row_iter(row_idx).collect();
        let left = &row[..col_idx];
        let right = &row[col_idx + 1..];

        let is_visible_via_left = left.iter().all(|other| height > **other);

        if is_visible_via_left {
            return true;
        }

        let is_visible_via_right = right.iter().all(|other| height > **other);

        if is_visible_via_right {
            return true;
        }

        let col: Vec<_> = self.col_iter(col_idx).collect();
        let above = &col[..row_idx];
        let below = &col[row_idx + 1..];

        let is_visible_via_above = above.iter().all(|other| height > **other);

        if is_visible_via_above {
            return true;
        }

        let is_visible_via_below = below.iter().all(|other| height > **other);

        if is_visible_via_below {
            return true;
        }

        false
    }

    fn scenic_score(&self, idx: usize) -> u32 {
        let (row_idx, col_idx) = self.to_row_col(idx);
        let height = self.items[row_idx][col_idx];

        let row: Vec<_> = self.row_iter(row_idx).collect();

        let left = &mut row.clone()[..col_idx];
        left.reverse();

        let right = &row[col_idx + 1..];

        let col: Vec<_> = self.col_iter(col_idx).collect();
        let above = &mut col.clone()[..row_idx];
        above.reverse();

        let below = &col[row_idx + 1..];

        calculate_viewing_distance(height, left)
            * calculate_viewing_distance(height, right)
            * calculate_viewing_distance(height, above)
            * calculate_viewing_distance(height, below)
    }
}

fn calculate_viewing_distance(height: u8, other_trees: &[&u8]) -> u32 {
    let mut count = 0;
    for other in other_trees.iter() {
        if **other >= height {
            count += 1;
            break;
        } else {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let count = count_visible(&input);

        assert_eq!(count, 21);
    }

    #[rstest]
    #[case(5, &[&3], 1)]
    #[case(5, &[&3, &5, &3], 2)]
    #[case(5, &[&5, &2], 1)]
    #[case(5, &[&1, &2], 2)]
    fn calculate_viewing_distance_case(
        #[case] height: u8,
        #[case] other_trees: &[&u8],
        #[case] expected: u32,
    ) {
        assert_eq!(calculate_viewing_distance(height, other_trees), expected);
    }

    #[rstest]
    #[case(7, 4)]
    #[case(17, 8)]
    fn scenic_score_example_case(#[case] idx: usize, #[case] expected: u32) {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");
        let grid = input.parse::<Grid<u8>>().unwrap();

        assert_eq!(grid.scenic_score(idx), expected);
    }

    #[test]
    fn max_scenic_score_example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        assert_eq!(max_scenic_score(&input), 8);
    }
}
