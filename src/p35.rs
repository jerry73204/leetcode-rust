use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        (match nums.binary_search(&target) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p35_test() {
        check(vec![1, 3, 5, 6], 5, 2);
        check(vec![1, 3, 5, 6], 2, 1);
        check(vec![1, 3, 5, 6], 7, 4);
        check(vec![7], 7, 0);
        check(vec![7], 5, 0);
        check(vec![7], 10, 1);
    }

    fn check(nums: Vec<i32>, target: i32, expect: i32) {
        assert_eq!(Solution::search_insert(nums, target), expect);
    }
}
