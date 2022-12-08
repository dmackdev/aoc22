use grid::Grid;

pub mod grid;

pub fn count_visible(input: &str) -> usize {
    let grid = input.parse::<Grid<u8>>().unwrap();

    grid.iter()
        .enumerate()
        .filter(|(idx, _)| grid.is_visible(*idx))
        .count()
}

trait Visible {
    fn is_visible(&self, idx: usize) -> bool;
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
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let count = count_visible(&input);

        assert_eq!(count, 21);
    }
}
