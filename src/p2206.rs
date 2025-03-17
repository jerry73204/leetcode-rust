//! # 2206. Divide Array Into Equal Pairs
//! [Submission](https://leetcode.com/problems/divide-array-into-equal-pairs/submissions/1576508518)

use crate::Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut odd_counts: usize = 0;
        let mut counts = [false; 501];

        for val in nums {
            let entry = &mut counts[val as usize];
            *entry = !*entry;

            if *entry {
                odd_counts += 1;
            } else {
                odd_counts -= 1;
            }
        }

        odd_counts == 0
    }
}
