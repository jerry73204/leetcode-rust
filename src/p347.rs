use crate::Solution;
use std::{cmp::Reverse, collections::HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();

        nums.into_iter().for_each(|val| {
            let count = counts.entry(val).or_insert_with(|| 0);
            *count += 1;
        });

        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_unstable_by_key(|(_val, count)| Reverse(*count));

        counts
            .into_iter()
            .take(k as usize)
            .map(|(val, _count)| val)
            .collect()
    }
}
