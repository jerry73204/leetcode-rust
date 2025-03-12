//! # 2529. Maximum Count of Positive Integer and Negative Integer
//! [Submission](https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/submissions/1571141589)

use crate::Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut neg = 0;

        for val in nums {
            if val > 0 {
                pos += 1;
            } else if val < 0 {
                neg += 1;
            }
        }

        pos.max(neg)
    }
}
