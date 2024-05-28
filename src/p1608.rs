use crate::Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by_key(|&v| Reverse(v));

        let mut il = 0;
        let mut ir = nums.len();

        while il + 1 < ir {
            let im = (il + ir) / 2;
            let x = (im + 1) as i32;

            if nums[im] < x {
                ir = im;
            } else if let Some(&next) = nums.get(im + 1) {
                if next < x {
                    return x;
                } else {
                    il = im;
                }
            } else {
                return x;
            }
        }

        let x = (il + 1) as i32;

        if nums[il] >= x {
            if let Some(&next) = nums.get(il + 1) {
                if next < x {
                    return x;
                }
            } else {
                return x;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1608_test() {
        check(vec![3, 5], 2);
        check(vec![0, 0], -1);
        check(vec![0, 4, 3, 0, 4], 3);
        check(vec![100, 99, 98], 3);
        check(vec![1], 1);
        check(vec![0, 1, 0], 1);
        check(vec![3, 1, 0], -1);
        check(vec![1, 1, 0], -1);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::special_array(nums), expect);
    }
}
