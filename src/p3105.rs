use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut inc_count = 1;
        let mut dec_count = 1;
        let mut max_inc_count = 1;
        let mut max_dec_count = 1;

        for window in nums.windows(2) {
            let [prev, curr] = *window else {
                unreachable!()
            };

            match prev.cmp(&curr) {
                Ordering::Less => {
                    inc_count += 1;
                    dec_count = 1;
                    max_inc_count = max_inc_count.max(inc_count);
                }
                Ordering::Equal => {
                    inc_count = 1;
                    dec_count = 1;
                }
                Ordering::Greater => {
                    inc_count = 1;
                    dec_count += 1;
                    max_dec_count = max_dec_count.max(dec_count);
                }
            }
        }

        max_inc_count.max(max_dec_count)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3105_test() {
        check(&[1, 4, 3, 3, 2], 2);
        check(&[3, 3, 3, 3], 1);
        check(&[3, 2, 1], 3);
    }

    fn check(nums: &[i32], expect: i32) {
        let answer = Solution::longest_monotonic_subarray(nums.to_vec());
        assert_eq!(answer, expect);
    }
}
