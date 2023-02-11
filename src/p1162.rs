use crate::Solution;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

impl Solution {
    pub fn max_distance(land: Vec<Vec<i32>>) -> i32 {
        let nrows = land.len();
        let ncols = land[0].len();
        let mut visited = Grid::new(nrows, ncols, false);

        let is_land = |(row, col): (usize, usize)| -> bool { land[row][col] != 0 };
        let traverse_iter = || (0..nrows).flat_map(|row| (0..ncols).map(move |col| (row, col)));
        let neighbors = |(row, col): (usize, usize)| {
            let top = row.checked_sub(1).map(|new_row| (new_row, col));
            let left = col.checked_sub(1).map(|new_col| (row, new_col));
            let bottom = (row <= nrows - 2).then(|| (row + 1, col));
            let right = (col <= ncols - 2).then(|| (row, col + 1));
            top.into_iter().chain(left).chain(bottom).chain(right)
        };

        let mut fronts: HashSet<_> = traverse_iter()
            .filter(|&pos| is_land(pos))
            .flat_map(|pos| {
                visited[pos] = true;
                neighbors(pos)
            })
            .filter(|&pos| !is_land(pos))
            .collect();

        if fronts.is_empty() {
            return -1;
        }

        let mut max_distance = 0;

        for distance in 0.. {
            let mut new_fronts = HashSet::new();

            for pos in fronts {
                let visited = &mut visited[pos];
                if *visited {
                    continue;
                }
                *visited = true;

                new_fronts.extend(neighbors(pos));
            }

            if new_fronts.is_empty() {
                max_distance = distance;
                break;
            }

            fronts = new_fronts;
        }

        max_distance
    }
}

struct Grid<T> {
    array: Vec<T>,
    ncols: usize,
}

impl<T> Grid<T> {
    pub fn new(nrows: usize, ncols: usize, init: T) -> Self
    where
        T: Clone,
    {
        let size = nrows * ncols;
        Self {
            array: vec![init; size],
            ncols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if col >= self.ncols {
            return None;
        }
        self.array.get(row * self.ncols + col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if col >= self.ncols {
            return None;
        }
        self.array.get_mut(row * self.ncols + col)
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
    fn p1162_test() {
        check(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], 2);
        check(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 4);
        check(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], -1);
        check(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], -1);
        check(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], 1);
    }

    fn check(grid: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::max_distance(grid), expect);
    }
}
