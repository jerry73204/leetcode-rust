use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
        #[derive(Debug)]
        struct State {
            pub val: i32,
            pub repeats: u32,
            pub sum_inclusive: i32,
            pub sum_exclusive: i32,
            pub is_end: bool,
        }

        impl Default for State {
            fn default() -> Self {
                Self {
                    val: i32::MIN,
                    repeats: u32::MAX,
                    sum_inclusive: i32::MIN,
                    sum_exclusive: i32::MIN,
                    is_end: true,
                }
            }
        }

        nums.sort_unstable();

        let mut states = {
            let mut uniq = Vec::with_capacity(nums.len());
            let mut iter = nums.into_iter();

            uniq.push(State {
                val: iter.next().unwrap(),
                repeats: 1,
                ..Default::default()
            });
            let mut last = uniq.last_mut().unwrap();

            for next_val in iter {
                if last.val == next_val {
                    last.repeats += 1;
                } else {
                    uniq.push(State {
                        val: next_val,
                        repeats: 1,
                        ..Default::default()
                    });
                    last = uniq.last_mut().unwrap();
                }
            }

            uniq
        };

        let first_val = states[0].val;
        let idx = match states.binary_search_by_key(&(first_val + k), |s| s.val) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        states[0..idx].iter_mut().for_each(|s| {
            s.sum_inclusive = 2i32.pow(s.repeats) - 1;
            s.sum_exclusive = 1;
        });

        let mut pi = 0;

        for ni in idx..states.len() {
            let curr = &states[ni];

            loop {
                let prev = &states[pi];

                match (prev.val + k).cmp(&curr.val) {
                    Ordering::Less => {
                        pi += 1;
                    }
                    Ordering::Equal => {
                        let sum_inclusive = prev.sum_exclusive * (2i32.pow(curr.repeats) - 1);
                        let sum_exclusive = prev.sum_inclusive + prev.sum_exclusive;

                        let prev = &mut states[pi];
                        prev.is_end = false;

                        let curr = &mut states[ni];
                        curr.sum_inclusive = sum_inclusive;
                        curr.sum_exclusive = sum_exclusive;
                        break;
                    }
                    Ordering::Greater => {
                        pi -= 1;

                        let curr = &mut states[ni];
                        curr.sum_inclusive = 2i32.pow(curr.repeats) - 1;
                        curr.sum_exclusive = 1;
                        break;
                    }
                }
            }
        }

        let sum: i32 = states
            .iter()
            .filter(|s| s.is_end)
            .map(|s| s.sum_inclusive + s.sum_exclusive)
            .product();

        sum - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2597_test() {
        check(vec![2, 4, 6], 2, 4);
        check(vec![1], 1, 1);
        check(vec![1, 2, 3, 3], 1, 8);
    }

    fn check(nums: Vec<i32>, k: i32, expect: i32) {
        assert_eq!(Solution::beautiful_subsets(nums, k), expect);
    }
}
