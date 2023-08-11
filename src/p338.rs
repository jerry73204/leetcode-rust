use crate::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).map(|v| v.count_ones() as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p338_test() {
        check(2, vec![0, 1, 1]);
        check(5, vec![0, 1, 1, 2, 1, 2]);
    }

    fn check(n: i32, expect: Vec<i32>) {
        assert_eq!(Solution::count_bits(n), expect);
    }
}
