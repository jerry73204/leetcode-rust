use crate::Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut slice = nums.as_slice();

        loop {
            let midx = ((slice.len() + 1) / 2) & !1;
            let mval = slice[midx];
            let lval = midx.checked_sub(1).map(|lidx| slice[lidx]);
            let rval = slice.get(midx + 1).copied();

            if Some(mval) == rval {
                slice = &slice[(midx + 2)..];
            } else if Some(mval) == lval {
                if let Some(new_idx) = midx.checked_sub(1) {
                    slice = &slice[..new_idx];
                } else {
                    return mval;
                }
            } else {
                return mval;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p540_test() {
        check(vec![1, 1, 2, 3, 3, 4, 4, 8, 8], 2);
        check(vec![3, 3, 7, 7, 10, 11, 11], 10);
        check(vec![2], 2);
        check(vec![1, 1, 2], 2);
        check(vec![1, 2, 2], 1);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::single_non_duplicate(nums), expect);
    }
}
