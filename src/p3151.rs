use crate::Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|window| {
            let [prev, curr] = *window else {
                unreachable!()
            };
            (prev ^ curr) & 1 != 0
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3151_test() {
        check(&[1], true);
        check(&[2, 1, 4], false);
        check(&[4, 3, 1, 6], false);
    }

    fn check(nums: &[i32], expect: bool) {
        assert_eq!(Solution::is_array_special(nums.to_vec()), expect);
    }
}
