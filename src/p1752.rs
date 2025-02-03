use crate::Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut found_dec = *nums.last().unwrap() > nums[0];

        for window in nums.windows(2) {
            let [prev, curr] = *window else {
                unreachable!()
            };
            if prev > curr {
                if found_dec {
                    return false;
                }
                found_dec = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1752_test() {
        check(&[3, 4, 5, 1, 2], true);
        check(&[2, 1, 3, 4], false);
        check(&[1, 2, 3], true);
    }

    fn check(nums: &[i32], expect: bool) {
        let answer = Solution::check(nums.to_vec());
        assert_eq!(answer, expect);
    }
}
