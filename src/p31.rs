use crate::Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // Find the first position `ix` where nums[ix] < nums[ix + 1]
        let Some((split_ix, _)) = nums
            .windows(2)
            .enumerate()
            .rev()
            .find(|(_, win)| win[0] < win[1])
        else {
            // If such this position does not exist, it's already a
            // lexicographically greatest permutation. Reverse the
            // array and return.
            nums.reverse();
            return;
        };

        // Get the pivot number at `ix` and numbers after the pivot.
        let (_, nums) = nums.split_at_mut(split_ix);
        let (pivot, nums) = nums.split_first_mut().unwrap();

        // Swap the pivot with a slightly greater number after the
        // pivot.
        let bin = nums.binary_search_by_key(&Reverse(*pivot + 1), |&val| Reverse(val));
        let swap_ix = match bin {
            Ok(ix) => ix,
            Err(ix) => ix - 1,
        };
        std::mem::swap(pivot, &mut nums[swap_ix]);

        // Sort the numbers after the pivot.
        nums.sort_unstable();
    }
}
