//! # 1780. Check if Number is a Sum of Powers of Three
//! [Submission](https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/submissions/1562122148)

use crate::Solution;

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            let rem = n % 3;
            if rem == 2 {
                return false;
            }
            n /= 3;
        }

        true
    }
}
