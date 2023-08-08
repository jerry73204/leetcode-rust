use crate::Solution;
use std::{cmp::Ordering::*, ops::RangeBounds, slice::SliceIndex};

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Search for rotation index
        let rot_idx = if is_rotated(&nums, ..) {
            let mut range = 0..nums.len();

            loop {
                let mid = (range.start + range.end) / 2;

                if is_rotated(&nums, range.start..mid) {
                    range = range.start..mid;
                } else if is_rotated(&nums, mid..range.end) {
                    range = mid..range.end;
                } else {
                    break Some(mid);
                }
            }
        } else {
            None
        };

        match rot_idx {
            Some(rot_idx) => match nums[0].cmp(&target) {
                Less => match nums[0..rot_idx].binary_search(&target) {
                    Ok(target_idx) => target_idx as i32,
                    Err(_) => -1,
                },
                Equal => 0,
                Greater => match nums[rot_idx..nums.len()].binary_search(&target) {
                    Ok(target_idx) => (rot_idx + target_idx) as i32,
                    Err(_) => -1,
                },
            },
            None => match nums.binary_search(&target) {
                Ok(target_idx) => target_idx as i32,
                Err(_) => -1,
            },
        }
    }
}

fn is_rotated<R>(nums: &[i32], range: R) -> bool
where
    R: RangeBounds<usize> + SliceIndex<[i32], Output = [i32]>,
{
    let slice = &nums[range];

    match (slice.first(), slice.last()) {
        (Some(first), Some(last)) => first > last,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p33_test() {
        check(vec![4, 5, 6, 7, 0, 1, 2], 0, 4);
        check(vec![4, 5, 6, 7, 0, 1, 2], 3, -1);
        check(vec![1], 0, -1);
        check(vec![1], 1, 0);
        check(vec![1, 2], 1, 0);
        check(vec![1, 2], 2, 1);
        check(vec![1, 2], 3, -1);
        check(vec![4, 3], 3, 1);
        check(vec![4, 3], 4, 0);
        check(vec![4, 3], 2, -1);
        check(vec![1, 3], 3, 1);
    }

    fn check(nums: Vec<i32>, target: i32, expect: i32) {
        assert_eq!(Solution::search(nums, target), expect);
    }
}
