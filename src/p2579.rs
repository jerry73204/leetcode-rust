//! 2579. Count Total Number of Colored Cells
//! [Submission](https://leetcode.com/problems/count-total-number-of-colored-cells/submissions/1563397207)
use crate::Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        2 * n * (n - 1) + 1
    }
}
