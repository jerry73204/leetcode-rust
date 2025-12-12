use crate::Solution;
use std::cmp::{max, min};

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut min_prefix_sums = vec![0i64; k];
        let mut prefix_sum = 0i64;

        let mut iter = nums.iter().map(|&val| val as i64);

        for (val, rem) in (&mut iter).zip(1..).take(k - 1) {
            prefix_sum += val;
            min_prefix_sums[rem] = prefix_sum;
        }

        let mut max_subarray_sum = i64::MIN;

        for (val, rem) in iter.zip((0..k).cycle()) {
            prefix_sum += val;
            let min_prefix_sum = &mut min_prefix_sums[rem];
            let subarray_sum = prefix_sum - *min_prefix_sum;
            max_subarray_sum = max(max_subarray_sum, subarray_sum);
            *min_prefix_sum = min(*min_prefix_sum, prefix_sum);
        }

        max_subarray_sum
    }
}
