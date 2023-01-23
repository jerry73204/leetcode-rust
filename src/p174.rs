use crate::Solution;
use std::{
    cmp,
    ops::{Index, IndexMut},
};

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let nrows = dungeon.len();
        let ncols = dungeon[0].len();
        let mut health = Grid::new(nrows, ncols, 0);
        health[(nrows - 1, ncols - 1)] = cmp::max(-dungeon[nrows - 1][ncols - 1], 0) + 1;

        for row in (0..(nrows - 1)).rev() {
            let col = ncols - 1;
            let min_health = cmp::max(health[(row + 1, col)] - dungeon[row][col], 1);
            health[(row, col)] = min_health;
        }

        for col in (0..(ncols - 1)).rev() {
            let row = nrows - 1;
            let min_health = cmp::max(health[(row, col + 1)] - dungeon[row][col], 1);
            health[(row, col)] = min_health;
        }

        (0..(nrows - 1))
            .rev()
            .flat_map(|row| (0..(ncols - 1)).rev().map(move |col| (row, col)))
            .for_each(|(row, col)| {
                let dun = dungeon[row][col];
                let right_health = cmp::max(health[(row + 1, col)] - dun, 1);
                let bottom_health = cmp::max(health[(row, col + 1)] - dun, 1);
                let min_health = cmp::min(right_health, bottom_health);
                health[(row, col)] = min_health;
            });

        health[(0, 0)]
    }
}

pub struct Grid<T> {
    array: Vec<T>,
    ncols: usize,
}

impl<T> Grid<T> {
    pub fn new(nrows: usize, ncols: usize, init: T) -> Self
    where
        T: Clone,
    {
        Self {
            array: vec![init; nrows * ncols],
            ncols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let Self {
            ref array, ncols, ..
        } = *self;
        if col >= ncols {
            return None;
        }

        array.get(row * ncols + col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        let Self {
            ref mut array,
            ncols,
            ..
        } = *self;
        if col >= ncols {
            return None;
        }

        array.get_mut(row * ncols + col)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p174_test() {
        check(vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]], 7);
        check(vec![vec![0]], 1);
    }

    fn check(dungeon: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::calculate_minimum_hp(dungeon), expect);
    }
}
