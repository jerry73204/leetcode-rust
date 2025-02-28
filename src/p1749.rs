//! # 1749. Maximum Absolute Sum of Any Subarray
//! [Submission](https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/submissions/1558320016)

use crate::Solution;
use std::cmp::{max, min};

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min_sum = 0;
        let mut max_sum = 0;
        let mut max_pos_diff = 0;
        let mut max_neg_diff = 0;

        for val in nums {
            sum += val;

            min_sum = min(min_sum, sum);
            max_sum = max(max_sum, sum);

            max_pos_diff = max(max_pos_diff, sum - min_sum);
            max_neg_diff = max(max_neg_diff, max_sum - sum);
        }

        max(max_pos_diff, max_neg_diff)
    }
}
