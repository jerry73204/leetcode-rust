use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut tab = vec![0; nums.len()];

        tab[0] = nums[0];
        tab[1] = nums[1];

        for (idx, num) in nums.iter().enumerate().skip(2) {
            tab[idx] = num + tab[0..(idx - 1)].iter().max().unwrap();
        }

        tab.iter().cloned().max().unwrap()
    }
}
