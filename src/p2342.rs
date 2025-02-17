//! # 2342. Max Sum of a Pair With Equal Sum of Digits
//! [Submission](https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/submissions/1546345776)

use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_vals = [None; 82];
        let mut max_sum = None;

        for &val in nums.iter() {
            let key = sum_of_digits(val);
            let max_val = &mut max_vals[key];

            if let Some(max_val) = max_val {
                let sum = *max_val + val;
                max_sum = max(max_sum, Some(sum));
                *max_val = max(*max_val, val);
            } else {
                *max_val = Some(val);
            }
        }

        max_sum.unwrap_or(-1)
    }
}

fn sum_of_digits(mut val: i32) -> usize {
    let mut sum = 0;

    while val > 0 {
        sum += val % 10;
        val /= 10;
    }

    sum as usize
}
