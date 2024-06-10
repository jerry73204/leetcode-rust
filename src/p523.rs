use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut residual = HashMap::with_capacity(nums.len());
        residual.insert(0, 0);
        nums.into_iter()
            .scan(0, |res, val| {
                *res = (*res + val) % k;
                Some(*res)
            })
            .zip(1..)
            .any(|(res, len)| {
                let prev = residual.entry(res).or_insert(len);
                (len - *prev) >= 2
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p523_test() {
        check(vec![23, 2, 4, 6, 7], 6, true);
        check(vec![23, 2, 6, 4, 7], 6, true);
        check(vec![23, 2, 6, 4, 7], 13, false);
        check(vec![1, 1, 1], 2, true);
        check(vec![1, 1, 1], 5, false);
    }

    fn check(nums: Vec<i32>, k: i32, expect: bool) {
        assert_eq!(Solution::check_subarray_sum(nums, k), expect);
    }
}
