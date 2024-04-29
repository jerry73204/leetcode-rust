use crate::Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let sum = nums.into_iter().reduce(|lhs, rhs| lhs ^ rhs).unwrap() ^ k;
        sum.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2997_test() {
        check(vec![2, 1, 3, 4], 1, 2);
        check(vec![2, 0, 2, 0], 0, 0);
        check(vec![1], 1, 0);
        check(vec![1], 3, 1);
    }

    fn check(nums: Vec<i32>, k: i32, expect: i32) {
        assert_eq!(Solution::min_operations(nums, k), expect);
    }
}
