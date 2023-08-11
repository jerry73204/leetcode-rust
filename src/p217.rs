use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for num in nums {
            if !set.insert(num) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p217_test() {
        check(vec![1, 2, 3, 1], true);
        check(vec![1, 2, 3, 4], false);
        check(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true);
    }

    fn check(nums: Vec<i32>, expect: bool) {
        assert_eq!(Solution::contains_duplicate(nums), expect);
    }
}
