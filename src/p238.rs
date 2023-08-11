use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let prefix_product = nums.iter().scan(1, |prod, &val| {
            let prev = *prod;
            *prod *= val;
            Some(prev)
        });
        let suffix_product: Vec<_> = nums
            .iter()
            .rev()
            .scan(1, |prod, &val| {
                let prev = *prod;
                *prod *= val;
                Some(prev)
            })
            .collect();

        prefix_product
            .into_iter()
            .zip(suffix_product.into_iter().rev())
            .map(|(lprod, rprod)| lprod * rprod)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p238_test() {
        check(vec![1, 2, 3, 4], vec![24, 12, 8, 6]);
        check(vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]);
    }

    fn check(nums: Vec<i32>, expect: Vec<i32>) {
        assert_eq!(Solution::product_except_self(nums), expect);
    }
}
