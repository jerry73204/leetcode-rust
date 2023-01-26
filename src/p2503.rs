use crate::Solution;
use std::{cmp::Reverse, collections::BinaryHeap, iter};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let nrows = grid.len();
        let ncols = grid[0].len();

        let mut query_indices: Vec<usize> = (0..queries.len()).collect();
        query_indices.sort_unstable_by_key(|&index| queries[index]);

        let compressed_queries: Vec<(i32, usize)> = {
            let mut iter = query_indices.iter().map(|&index| queries[index]);
            let mut output = vec![];
            let mut curr = iter.next().unwrap();
            let mut count = 1;

            for val in iter {
                if val == curr {
                    count += 1;
                } else {
                    output.push((curr, count));
                    curr = val;
                    count = 1;
                }
            }

            output.push((curr, count));
            output
        };

        let mut visited_grid = Grid::new(nrows, ncols, false);
        let mut count = 0;
        let mut fronts: BinaryHeap<Reverse<Candidate>> = BinaryHeap::new();
        fronts.push(Reverse(Candidate {
            value: grid[0][0],
            row: 0,
            col: 0,
        }));

        let counts: Vec<_> = compressed_queries
            .into_iter()
            .flat_map(|(query, run_len)| {
                while let Some(Reverse(curr)) = fronts.peek() {
                    let Candidate { row, col, .. } = *curr;
                    if query <= grid[row][col] {
                        break;
                    }

                    fronts.pop();

                    let visited = visited_grid.get_mut(row, col).unwrap();
                    if *visited {
                        continue;
                    }

                    *visited = true;
                    count += 1;

                    let right = {
                        let new_row = row + 1;
                        (new_row < nrows).then(|| (new_row, col))
                    };
                    let lower = {
                        let new_col = col + 1;
                        (new_col < ncols).then(|| (row, new_col))
                    };
                    let left = row.checked_sub(1).map(|new_row| (new_row, col));
                    let upper = col.checked_sub(1).map(|new_col| (row, new_col));

                    let candidates = right.into_iter().chain(lower).chain(left).chain(upper).map(
                        |(new_row, new_col)| {
                            Reverse(Candidate {
                                value: grid[new_row][new_col],
                                row: new_row,
                                col: new_col,
                            })
                        },
                    );
                    fronts.extend(candidates);
                }

                iter::repeat(count).take(run_len)
            })
            .collect();

        let mut answer = vec![0; queries.len()];
        query_indices
            .into_iter()
            .enumerate()
            .for_each(|(src, dst)| {
                answer[dst] = counts[src];
            });

        answer
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
        let array = vec![init; nrows * ncols];
        Self { array, ncols }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if col >= self.ncols {
            return None;
        }
        let index = row * self.ncols + col;
        self.array.get_mut(index)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate {
    pub value: i32,
    pub row: usize,
    pub col: usize,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2503_test() {
        check(
            vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            vec![5, 6, 2],
            vec![5, 8, 1],
        );
        check(
            vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            vec![5, 5, 5, 6, 6, 2, 2, 2, 2],
            vec![5, 5, 5, 8, 8, 1, 1, 1, 1],
        );
        check(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3], vec![0]);
    }

    fn check(grid: Vec<Vec<i32>>, queries: Vec<i32>, expect: Vec<i32>) {
        assert_eq!(Solution::max_points(grid, queries), expect);
    }
}
