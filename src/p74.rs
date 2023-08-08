use crate::Solution;
use std::cmp::Ordering::*;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let nrows = matrix.len();
        let ncols = matrix[0].len();
        let nelems = nrows * ncols;

        let idx1to2 = |idx: usize| {
            let row = idx / ncols;
            let col = idx % ncols;
            (row, col)
        };

        let get_val = |idx: usize| {
            let (row, col) = idx1to2(idx);
            matrix[row][col]
        };

        let mut range = 0..nelems;

        loop {
            match range.len() {
                0 => break false,
                1 => break get_val(range.start) == target,
                _ => {
                    let mid = (range.start + range.end) / 2;
                    match get_val(mid).cmp(&target) {
                        Less => {
                            range = (mid + 1)..range.end;
                        }
                        Equal => break true,
                        Greater => {
                            range = range.start..mid;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p74_test() {
        check(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
            true,
        );
        check(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13,
            false,
        );
    }

    fn check(matrix: Vec<Vec<i32>>, target: i32, expect: bool) {
        assert_eq!(Solution::search_matrix(matrix, target), expect);
    }
}
