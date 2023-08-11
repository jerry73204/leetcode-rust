use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tab = vec![0; nums.len()];
        let mut len = 0;

        for val in nums {
            if let Err(idx) = tab[0..len].binary_search(&val) {
                if idx == len {
                    tab[len] = val;
                    len += 1;
                } else {
                    tab[idx] = val;
                }
            }
        }

        len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p300_test() {
        check(vec![10, 9, 2, 5, 3, 7, 101, 18], 4);
        check(vec![0, 1, 0, 3, 2, 3], 4);
        check(vec![7, 7, 7, 7, 7, 7, 7], 1);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::length_of_lis(nums), expect);
    }
}
