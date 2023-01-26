use crate::Solution;
use std::collections::VecDeque;
use std::{array, iter};

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let lower = lower as i64;
        let upper = upper as i64;
        let n_sums = nums.len() + 1;
        let bound_size = upper - lower;

        let prefix_sum = {
            let iter = nums.into_iter().scan(0i64, |sum, val| {
                *sum += val as i64;
                Some(*sum)
            });
            iter::once(0).chain(iter)
        };

        let mut events: Vec<_> = prefix_sum
            .enumerate()
            .flat_map(|(index, val)| {
                array::IntoIter::new([
                    Event {
                        value: val,
                        is_phantom: false,
                        index,
                    },
                    Event {
                        value: val - lower,
                        is_phantom: true,
                        index,
                    },
                ])
            })
            .collect();
        events.sort_unstable();

        let mut values: VecDeque<Event> = VecDeque::with_capacity(n_sums);
        let mut prefix_count = FenwickTree::new(n_sums);
        let mut count = 0;

        for evt in events {
            if evt.is_phantom {
                let lower_bound = evt.value - bound_size;

                while let Some(front) = values.front() {
                    if front.value < lower_bound {
                        let front = values.pop_front().unwrap();
                        let yes = prefix_count.add(front.index, -1);
                        assert!(yes);
                    } else {
                        break;
                    }
                }

                count += prefix_count.prefix_sum(evt.index).unwrap();
            } else {
                let yes = prefix_count.add(evt.index, 1);
                values.push_back(evt);
                debug_assert!(yes);
            }
        }

        count as i32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    pub value: i64,
    pub is_phantom: bool,
    pub index: usize,
}

struct FenwickTree {
    nodes: Vec<i64>,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![0; size],
        }
    }

    pub fn prefix_sum(&self, len: usize) -> Option<i64> {
        let index = match len.checked_sub(1) {
            Some(index) => index,
            None => {
                return Some(0);
            }
        };

        if index >= self.nodes.len() {
            return None;
        }

        let indices = iter::successors(Some(index), |&prev| {
            (prev != 0).then(|| {
                let lsb = 1 << prev.trailing_zeros();
                prev ^ lsb
            })
        });

        let prefix_sum = indices.map(|index| self.nodes[index]).sum();
        Some(prefix_sum)
    }

    #[must_use]
    pub fn add(&mut self, index: usize, inc: i64) -> bool {
        if index >= self.nodes.len() {
            return false;
        } else if index == 0 {
            self.nodes[0] += inc;
            return true;
        }

        let mut index = index;

        while let Some(value) = self.nodes.get_mut(index) {
            *value += inc;
            let lsb = 1 << index.trailing_zeros();
            index += lsb;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;
    use crate::Solution;

    #[test]
    fn p327_test() {
        check(vec![-2, 5, -1], -2, 2, 3);
        check(vec![0], 0, 0, 1);
        check(vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864, 3);
    }

    fn check(nums: Vec<i32>, lower: i32, upper: i32, expect: i32) {
        assert_eq!(Solution::count_range_sum(nums, lower, upper), expect);
    }

    #[test]
    fn p327_fenwick_tree_test() {
        {
            let mut tree = FenwickTree::new(0);
            assert_eq!(tree.prefix_sum(0), Some(0));
            assert_eq!(tree.prefix_sum(1), None);
            assert!(!tree.add(0, 1));
        }

        {
            let mut tree = FenwickTree::new(2);
            assert_eq!(tree.prefix_sum(0), Some(0));
            assert_eq!(tree.prefix_sum(1), Some(0));
            assert_eq!(tree.prefix_sum(2), Some(0));
            assert_eq!(tree.prefix_sum(3), None);

            assert!(tree.add(1, 1));

            assert_eq!(tree.prefix_sum(0), Some(0));
            assert_eq!(tree.prefix_sum(1), Some(0));
            assert_eq!(tree.prefix_sum(2), Some(1));
            assert_eq!(tree.prefix_sum(3), None);

            assert!(tree.add(0, -2));

            assert_eq!(tree.prefix_sum(0), Some(0));
            assert_eq!(tree.prefix_sum(1), Some(-2));
            assert_eq!(tree.prefix_sum(2), Some(-1));
            assert_eq!(tree.prefix_sum(3), None);
        }

        {
            let mut tree = FenwickTree::new(17);
            assert_eq!(tree.prefix_sum(0), Some(0));
            assert_eq!(tree.prefix_sum(16), Some(0));
            assert_eq!(tree.prefix_sum(17), Some(0));
            assert_eq!(tree.prefix_sum(18), None);

            assert!(tree.add(8, 7));

            assert_eq!(tree.prefix_sum(0), Some(0));
            assert_eq!(tree.prefix_sum(8), Some(0));
            assert_eq!(tree.prefix_sum(9), Some(7));
            assert_eq!(tree.prefix_sum(17), Some(7));

            assert!(tree.add(0, 3));

            assert_eq!(tree.prefix_sum(0), Some(0));
            assert_eq!(tree.prefix_sum(1), Some(3));
            assert_eq!(tree.prefix_sum(9), Some(10));
        }
    }
}
