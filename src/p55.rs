use crate::Solution;
use std::cmp;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let reach: Vec<_> = nums
            .iter()
            .enumerate()
            .map(|(idx, &dist)| idx + dist as usize)
            .collect();
        let max_reach: Vec<_> = reach
            .iter()
            .scan(0, |max, &reach| {
                *max = cmp::min(cmp::max(*max, reach), nums.len() - 1);
                Some(*max)
            })
            .collect();

        let mut idx = 0;

        while idx != nums.len() - 1 {
            let new_idx = max_reach[idx];

            if new_idx == idx {
                return false;
            }

            idx = new_idx;
        }

        true
    }
}
