use crate::Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut count = vec![0; (target + 1) as usize];
        count[0] = 1;

        for sum in 1..=target {
            count[sum as usize] = nums
                .iter()
                .map(|val| sum - val)
                .filter(|&prev_sum| prev_sum >= 0)
                .map(|prev_sum| count[prev_sum as usize])
                .sum();
        }

        count[target as usize]
    }
}
