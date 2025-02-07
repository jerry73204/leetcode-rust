use crate::Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter();
        let mut prev = iter.next().unwrap();
        let mut sum = prev;
        let mut max_sum = i32::MIN;

        for curr in iter {
            if prev < curr {
                sum += curr;
            } else {
                max_sum = max_sum.max(sum);
                sum = curr;
            }

            prev = curr;
        }
        max_sum = max_sum.max(sum);

        max_sum
    }
}
