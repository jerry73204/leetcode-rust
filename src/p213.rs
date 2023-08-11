use crate::Solution;
use std::cmp;

impl Solution {
    // Rename to rob for submission
    pub fn rob2(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 1 {
            return nums[0];
        }

        cmp::max(solve(&nums[0..(len - 1)]), solve(&nums[1..len]))
    }
}

fn solve(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut tab = vec![0; nums.len()];
    tab[0] = nums[0];
    tab[1] = nums[1];

    for (idx, &num) in nums.iter().enumerate().skip(2) {
        tab[idx] = num + tab[0..(idx - 1)].iter().cloned().max().unwrap();
    }

    tab.iter().cloned().max().unwrap()
}
