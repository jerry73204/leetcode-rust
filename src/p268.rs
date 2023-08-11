use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let all = (0..=(nums.len() as i32)).fold(0, |xor, val| xor ^ val);
        let miss1 = nums.iter().fold(0, |xor, &val| xor ^ val);
        all ^ miss1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p268_test() {
        check(vec![3, 0, 1], 2);
        check(vec![0, 1], 2);
        check(vec![9, 6, 4, 2, 3, 5, 7, 0, 1], 8);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::missing_number(nums), expect);
    }
}
