use crate::Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut iter = nums.into_iter();
        let mut level = iter.next().unwrap();
        let mut moves = 0;

        for val in iter {
            level = val.max(level + 1);
            moves += level - val;
        }

        moves
    }
}
