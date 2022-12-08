use std::{marker::PhantomData, slice::Iter, str::FromStr};

pub struct Grid<'a, T> {
    items: Vec<Vec<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Grid<'a, T> {
    fn new(items: Vec<Vec<T>>) -> Grid<'a, T> {
        Grid {
            items,
            _marker: PhantomData,
        }
    }

    fn num_rows(&self) -> usize {
        self.items.len()
    }

    fn num_columns(&self) -> usize {
        self.items[0].len()
    }

    fn row_iter(&self, row_idx: usize) -> Iter<T> {
        self.items[row_idx].iter()
    }

    fn iter(&self) -> GridIterator<'_, T> {
        GridIterator {
            grid: self,
            curr_idx: 0,
        }
    }

    fn col_iter(&self, col_idx: usize) -> GridColumnIterator<'_, T> {
        GridColumnIterator {
            grid: self,
            col_idx,
            curr_row_idx: 0,
        }
    }
}

impl<'a, T> FromStr for Grid<'a, T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Vec<Vec<T>> = Vec::new();

        for line in s.lines() {
            let row = line
                .chars()
                .map(|c| c.to_string().parse::<T>())
                .collect::<Result<Vec<T>, _>>()?;

            items.push(row);
        }

        Ok(Grid::new(items))
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<'a, T>,
    curr_idx: usize,
}

impl<'a, T: 'a> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let joined_items: Vec<&T> = self.grid.items.iter().flatten().collect();

        if self.curr_idx < joined_items.len() {
            let result = Some(joined_items[self.curr_idx]);
            self.curr_idx += 1;
            result
        } else {
            None
        }
    }
}

pub struct GridColumnIterator<'a, T> {
    grid: &'a Grid<'a, T>,
    col_idx: usize,
    curr_row_idx: usize,
}

impl<'a, T> Iterator for GridColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row_idx < self.grid.num_rows() {
            let result = Some(&self.grid.items[self.curr_row_idx][self.col_idx]);
            self.curr_row_idx += 1;
            result
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "\
123
456
789";

        let grid = input.parse::<Grid<u8>>().unwrap();

        assert_eq!(
            grid.items,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
    }

    #[test]
    fn grid_iter() {
        let items = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(items);

        let mut iter = grid.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn grid_col_iter() {
        let items = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(items);

        let mut col_iter = grid.col_iter(1);

        assert_eq!(col_iter.next(), Some(&2));
        assert_eq!(col_iter.next(), Some(&5));
        assert_eq!(col_iter.next(), Some(&8));
        assert_eq!(col_iter.next(), None);
    }
}
