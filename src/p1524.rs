//! # 1524. Number of Sub-arrays With Odd Sum
//! [Submission](https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/submissions/1558337331)

use crate::Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum_oddity = false;
        let mut num_odd = 0;
        let mut num_even = 1;
        let mut count = 0;

        for val in arr {
            let val_oddity = (val & 1) != 0;
            sum_oddity ^= val_oddity;

            if sum_oddity {
                count += num_even;
                num_odd += 1;
            } else {
                count += num_odd;
                num_even += 1;
            }

            count %= 1_000_000_007;
        }

        count
    }
}
