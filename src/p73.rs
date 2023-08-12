use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // Find rows and columns to erase.
        let mut erase_rows = HashSet::new();
        let mut erase_cols = HashSet::new();

        matrix
            .iter()
            .enumerate()
            .flat_map(|(ridx, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(cidx, &elem)| (ridx, cidx, elem))
            })
            .filter(|(_, _, elem)| *elem == 0)
            .for_each(|(ridx, cidx, _)| {
                erase_rows.insert(ridx);
                erase_cols.insert(cidx);
            });

        // Erase selected rows.
        for ridx in erase_rows {
            matrix[ridx].iter_mut().for_each(|elem| {
                *elem = 0;
            });
        }

        // Erase selected columns.
        for cidx in erase_cols {
            for row in &mut *matrix {
                row[cidx] = 0;
            }
        }
    }
}
