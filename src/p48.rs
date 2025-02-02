use crate::Solution;
use std::{cmp::Ordering::*, mem};

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let swap = |matrix: &mut Vec<Vec<i32>>, pos1, pos2| {
            let (e1, e2) = get_two_elems_of_matrix(matrix, pos1, pos2);
            mem::swap(e1, e2);
        };

        let size = matrix.len();

        for pad in 0..(size / 2) {
            let rt = pad;
            let rb = size - pad - 1;
            let cl = pad;
            let cr = size - pad - 1;

            let steps = size - 2 * pad - 1;

            for st in 0..steps {
                let pos1 = (rt, cl + st);
                let pos2 = (rb - st, cl);
                let pos3 = (rb, cr - st);
                let pos4 = (rt + st, cr);

                swap(matrix, pos1, pos2);
                swap(matrix, pos2, pos3);
                swap(matrix, pos3, pos4);
            }
        }
    }
}

fn get_two_elems_of_matrix(
    matrix: &mut Vec<Vec<i32>>,
    (r1, c1): (usize, usize),
    (r2, c2): (usize, usize),
) -> (&mut i32, &mut i32) {
    match (r1 == r2, c1 == c2) {
        (true, true) => panic!(),
        (true, false) => get_two_elems_of_row(&mut matrix[r1], c1, c2),
        (false, _) => {
            let (row1, row2) = get_two_rows_of_matrix(matrix, r1, r2);
            (&mut row1[c1], &mut row2[c2])
        }
    }
}

#[allow(clippy::ptr_arg)]
fn get_two_rows_of_matrix(
    matrix: &mut Vec<Vec<i32>>,
    r1: usize,
    r2: usize,
) -> (&mut Vec<i32>, &mut Vec<i32>) {
    match r1.cmp(&r2) {
        Less => {
            let (upper, lower) = matrix.split_at_mut(r2);
            (&mut upper[r1], &mut lower[0])
        }
        Equal => panic!(),
        Greater => {
            let (upper, lower) = matrix.split_at_mut(r1);
            (&mut lower[0], &mut upper[r2])
        }
    }
}

#[allow(clippy::ptr_arg)]
fn get_two_elems_of_row(row: &mut Vec<i32>, c1: usize, c2: usize) -> (&mut i32, &mut i32) {
    match c1.cmp(&c2) {
        Less => {
            let (left, right) = row.split_at_mut(c2);
            (&mut left[c1], &mut right[0])
        }
        Equal => panic!(),
        Greater => {
            let (left, right) = row.split_at_mut(c1);
            (&mut right[0], &mut left[c2])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p48_test() {
        check(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
        );
        check(
            vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ],
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ],
        );
    }

    fn check(mut matrix: Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expect);
    }
}
