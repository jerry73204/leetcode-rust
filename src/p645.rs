use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut nums_iter = nums.into_iter().peekable();
        let mut expect_iter = (1..).peekable();
        let mut repeat = None;
        let mut missing = None;

        loop {
            let expect = *expect_iter.peek().unwrap();
            let val = match nums_iter.peek() {
                Some(&val) => val,
                None => {
                    missing = Some(expect);
                    break;
                }
            };

            match expect.cmp(&val) {
                Ordering::Less => {
                    expect_iter.next();
                    missing = Some(expect);
                    if repeat.is_some() {
                        break;
                    }
                }
                Ordering::Equal => {
                    nums_iter.next();
                    expect_iter.next();
                }
                Ordering::Greater => {
                    nums_iter.next();
                    repeat = Some(val);
                    if missing.is_some() {
                        break;
                    }
                }
            }
        }

        vec![repeat.unwrap(), missing.unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p645_test() {
        check(vec![1, 2, 2, 4], [2, 3]);
        check(vec![1, 1], [1, 2]);
    }

    fn check(nums: Vec<i32>, expect: [i32; 2]) {
        assert_eq!(Solution::find_error_nums(nums), expect);
    }
}
