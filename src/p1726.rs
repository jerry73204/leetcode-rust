use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut product_count: HashMap<i32, i32> = HashMap::with_capacity(n * (n - 1) / 2);

        for i in 0..n {
            for j in (i + 1)..n {
                let product = nums[i] * nums[j];
                *product_count.entry(product).or_default() += 1;
            }
        }

        product_count
            .into_values()
            .map(|n_pairs| n_pairs * (n_pairs - 1) * 4)
            .sum()
    }
}
