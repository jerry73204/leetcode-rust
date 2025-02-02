use crate::Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0; 3];

        for &val in nums.iter() {
            counts[val as usize] += 1;
        }

        let [c0, c1, _] = counts;
        nums[0..c0].fill(0);
        nums[c0..(c0 + c1)].fill(1);
        nums[(c0 + c1)..].fill(2);
    }
}
