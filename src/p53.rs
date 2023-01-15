use crate::Solution;
use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut min_prefix_sum = 0;
        let mut prefix_sum = 0;
        let mut max_subarray_sum = 0;

        for &val in &nums {
            prefix_sum += val;
            min_prefix_sum = cmp::min(min_prefix_sum, prefix_sum);
            max_subarray_sum = cmp::max(max_subarray_sum, prefix_sum - min_prefix_sum);
        }

        if max_subarray_sum > 0 {
            max_subarray_sum
        } else {
            nums.into_iter().max().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p52_test() {
        check(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6);
        check(vec![1], 1);
        check(vec![5, 4, -1, 7, 8], 23);
        check(vec![-1], -1);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::max_sub_array(nums), expect);
    }
}
