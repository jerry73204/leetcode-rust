use crate::Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        const MAX_BITS: usize = 31;

        let k_bits: Vec<bool> = (0..i32::BITS)
            .map(|nth| {
                let bit = 1 << nth;
                (k & bit) != 0
            })
            .collect();

        let mut runs = [0; 31];
        let mut min_diff = i32::MAX;

        for (val, len) in nums.into_iter().zip(1..) {
            (0..MAX_BITS)
                .map(|nth| 1 << nth)
                .map(|bit| (val & bit) != 0)
                .zip(&mut runs)
                .for_each(|(bit, run)| {
                    if bit {
                        *run += 1;
                    } else {
                        *run = 0;
                    }
                });

            let mut run_range = (1, len);
            let mut iter = runs.iter().zip(&k_bits).rev();

            let run = loop {
                let (start, end) = &mut run_range;

                let (&run, &bit) = match iter.next() {
                    Some(next) => next,
                    None => return 0,
                };

                if bit {
                    if run >= *start {
                        *end = std::cmp::min(*end, run);
                    } else {
                        break *start - 1;
                    }
                } else if run < *end {
                    *start = std::cmp::max(*start, run + 1);
                } else {
                    break *end;
                }
            };

            let diff = if run == 0 {
                let diff = k - range_and(&runs, 1);
                debug_assert!(diff > 0);
                diff
            } else if run == len {
                let diff = range_and(&runs, run) - k;
                debug_assert!(diff > 0);
                diff
            } else {
                let diff1 = k - range_and(&runs, run + 1);
                let diff2 = range_and(&runs, run) - k;
                debug_assert!(diff1 > 0);
                debug_assert!(diff2 > 0);
                std::cmp::min(diff1, diff2)
            };

            min_diff = std::cmp::min(min_diff, diff);
        }

        min_diff
    }
}

fn range_and(runs: &[usize], len: usize) -> i32 {
    runs.iter().enumerate().fold(0, |acc, (nth, &run)| {
        let bit = if len <= run { 1 << nth } else { 0 };
        acc | bit
    })
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3171_test() {
        check(vec![1, 2, 4, 5], 3, 1);
        check(vec![1, 2, 1, 2], 2, 0);
        check(vec![1], 10, 9);
        check(vec![8], 10, 2);
        check(vec![1, 2, 4, 8, 16], 7, 1);
    }

    fn check(nums: Vec<i32>, k: i32, expect: i32) {
        assert_eq!(Solution::minimum_difference(nums, k), expect);
    }
}
