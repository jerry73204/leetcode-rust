use crate::Solution;
use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut iter = prices.iter().copied();
        let first = iter.next().unwrap();

        iter.scan(first, |min, val| {
            let diff = val - *min;
            *min = cmp::min(*min, val);
            Some(diff)
        })
        .filter(|&diff| diff > 0)
        .max()
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p121_test() {
        check(vec![7, 1, 5, 3, 6, 4], 5);
        check(vec![7, 6, 4, 3, 1], 0);
        check(vec![7], 0);
        check(vec![1, 6], 5);
        check(vec![1, 6, 3], 5);
    }

    fn check(prices: Vec<i32>, expect: i32) {
        assert_eq!(Solution::max_profit(prices), expect);
    }
}
