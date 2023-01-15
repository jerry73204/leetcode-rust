use crate::Solution;
use std::cmp::{self, Reverse};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut height: Vec<_> = height
            .into_iter()
            .enumerate()
            .map(|(index, height)| Height { height, index })
            .collect();
        height.sort_unstable_by_key(|item| Reverse(item.height));

        let mut iter = height.into_iter();
        let mut min_idx = iter.next().unwrap().index;
        let mut max_idx = min_idx;

        iter.map(|item| {
            let Height {
                height: water_level,
                index: this_idx,
            } = item;
            let dist_for_min = abs_diff(this_idx, min_idx);
            let dist_for_max = abs_diff(this_idx, max_idx);
            let max_dist = cmp::max(dist_for_min, dist_for_max) as i32;
            min_idx = cmp::min(min_idx, this_idx);
            max_idx = cmp::max(max_idx, this_idx);
            max_dist * water_level
        })
        .max()
        .unwrap()
    }
}

struct Height {
    height: i32,
    index: usize,
}

fn abs_diff(lhs: usize, rhs: usize) -> usize {
    lhs.checked_sub(rhs).unwrap_or_else(|| rhs - lhs)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p11_test() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
        assert_eq!(Solution::max_area(vec![1, 2, 4, 3]), 4);
    }
}
