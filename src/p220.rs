use crate::Solution;
use std::{cmp, iter};

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let index_diff = index_diff as usize;

        let mut nums: Vec<Num> = nums
            .into_iter()
            .enumerate()
            .map(|(index, value)| Num { value, index })
            .collect();
        nums.sort_unstable();

        let mut indices = FenwickTree::new(nums.len());
        let mut lidx = 0;

        for rnum in &nums {
            indices.add(rnum.index, 1);

            while nums[lidx].value + value_diff < rnum.value {
                indices.add(nums[lidx].index, -1);
                lidx += 1;
            }

            let lower = rnum.index.saturating_sub(index_diff);
            let upper = cmp::min(rnum.index + index_diff, nums.len() - 1);

            let n_indices = indices.range_sum(lower, upper + 1).unwrap();
            if n_indices >= 2 {
                return true;
            }
        }

        false
    }
}

struct FenwickTree {
    nodes: Vec<i32>,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![0; size],
        }
    }

    pub fn prefix_sum(&self, len: usize) -> Option<i32> {
        let nodes = &self.nodes;

        let idx = match len.checked_sub(1) {
            Some(idx) => idx,
            None => return Some(0),
        };

        if idx >= nodes.len() {
            return None;
        }

        let sum = iter::successors(Some(idx), |&prev| {
            (prev != 0).then(|| prev ^ (1 << prev.trailing_zeros()))
        })
        .map(|idx| nodes[idx])
        .sum();
        Some(sum)
    }

    pub fn range_sum(&self, llen: usize, rlen: usize) -> Option<i32> {
        let (mut lidx, mut ridx) = match (llen.checked_sub(1), rlen.checked_sub(1)) {
            (None, None) => return Some(0),
            (None, Some(_)) => return self.prefix_sum(rlen),
            (Some(_), None) => {
                let sum = self.prefix_sum(llen)?;
                return Some(-sum);
            }
            (Some(lidx), Some(ridx)) => (lidx, ridx),
        };

        let nodes = &self.nodes;

        if lidx >= self.nodes.len() || ridx >= self.nodes.len() {
            return None;
        }

        let mut sum = 0;

        while lidx < ridx {
            sum += nodes[ridx];
            ridx ^= 1 << ridx.trailing_zeros();
        }

        while lidx > ridx {
            sum -= nodes[lidx];
            lidx ^= 1 << lidx.trailing_zeros();
        }

        Some(sum)
    }

    pub fn add(&mut self, mut index: usize, value: i32) {
        let nodes = &mut self.nodes;

        if index == 0 {
            nodes[0] += value;
            return;
        }

        while index < nodes.len() {
            nodes[index] += value;
            index += 1 << index.trailing_zeros();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Num {
    pub value: i32,
    pub index: usize,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p220_test() {
        check(vec![1, 2, 3, 1], 3, 0, true);
        check(vec![1, 5, 9, 1, 5, 9], 2, 3, false);
        check(vec![1], 3, 0, false);
        check(vec![1, 1], 1, 0, true);
        check(vec![1, 10, 2], 1, 1, false);
        check(vec![1, 10, 2], 2, 1, true);
        check(vec![1, 1], 0, 1, false);
        check(vec![-3, 3], 2, 4, false);
    }

    fn check(nums: Vec<i32>, index_diff: i32, value_diff: i32, expect: bool) {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff,),
            expect
        );
    }
}
