use crate::Solution;
use std::cmp;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter();
        let first = iter.next().unwrap();

        let mut max_val = first;
        let mut max_pos = MaxSubarraySum::new(first);
        let mut max_neg = MaxSubarraySum::new(-first);

        for val in iter {
            max_val = cmp::max(max_val, val);
            max_pos.push(val);
            max_neg.push(-val);
        }

        let max_subarray_sum = max_pos.max_diff;
        let min_subarray_sum = -max_neg.max_diff;

        let max_circular_subarray_sum = cmp::max(max_subarray_sum, max_pos.sum - min_subarray_sum);

        if max_circular_subarray_sum == 0 {
            debug_assert_eq!(max_pos.sum, min_subarray_sum);
            max_val
        } else {
            debug_assert!(max_circular_subarray_sum > 0);
            max_circular_subarray_sum
        }
    }
}

struct MaxSubarraySum {
    sum: i32,
    min_sum: i32,
    max_diff: i32,
}

impl MaxSubarraySum {
    pub fn new(first: i32) -> Self {
        let sum = first;
        let min_sum = cmp::min(first, 0);
        let max_diff = sum - min_sum;

        Self {
            sum,
            min_sum,
            max_diff,
        }
    }

    pub fn push(&mut self, val: i32) {
        self.sum += val;
        self.min_sum = cmp::min(self.min_sum, self.sum);
        self.max_diff = cmp::max(self.max_diff, self.sum - self.min_sum);
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p918_test() {
        check(vec![1, -2, 3, -2], 3);
        check(vec![5, -3, 5], 10);
        check(vec![-3, -2, -3], -2);
        check(vec![-1], -1);
        check(vec![0], 0);
        check(vec![1], 1);
        check(vec![1, -3, 1], 2);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::max_subarray_sum_circular(nums), expect);
    }
}
