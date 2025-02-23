//! # 2364. Count Number of Bad Pairs
//!
//! [Submission](https://leetcode.com/problems/count-number-of-bad-pairs/submissions/1553111394)

use crate::Solution;

impl Solution {
    pub fn count_bad_pairs(mut nums: Vec<i32>) -> i64 {
        let n_pairs = |n: i64| n * (n - 1) / 2;

        for (num, sub) in nums.iter_mut().zip(0i32..) {
            *num -= sub;
        }
        nums.sort_unstable();

        let n_good_pairs: i64 = nums
            .chunk_by(|prev, curr| prev == curr)
            .map(|run| n_pairs(run.len() as i64))
            .sum();
        n_pairs(nums.len() as i64) - n_good_pairs
    }
}
