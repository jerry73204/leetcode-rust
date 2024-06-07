use crate::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let cap = arr.len();
        let mut counts = HashMap::with_capacity(cap);

        for val in arr {
            *counts.entry(val).or_insert(0) += 1;
        }

        let mut set = HashSet::with_capacity(cap);
        for &count in counts.values() {
            if !set.insert(count) {
                return false;
            }
        }

        true
    }
}
