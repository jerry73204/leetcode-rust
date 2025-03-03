//! # 2161. Partition Array According to Given Pivot
//! [Submission](https://leetcode.com/problems/partition-array-according-to-given-pivot/submissions/1560898884)

use crate::Solution;
use std::{cmp::Ordering, iter::repeat};

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut num_greater = 0;
        let mut arranged = Vec::with_capacity(nums.len());

        for &val in &nums {
            match val.cmp(&pivot) {
                Ordering::Less => arranged.push(val),
                Ordering::Equal => {}
                Ordering::Greater => num_greater += 1,
            }
        }

        let num_less = arranged.len();
        let num_equal = nums.len() - num_less - num_greater;

        arranged.extend(repeat(pivot).take(num_equal));
        arranged.extend(nums.iter().filter(|&&val| val > pivot));

        arranged
    }
}
