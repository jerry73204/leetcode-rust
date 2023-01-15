use crate::Solution;
use std::mem;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n_pos = {
            let mut lidx = 0;
            let mut ridx = nums.len();

            while lidx < ridx {
                let value = nums[lidx];

                if value <= 0 {
                    ridx -= 1;

                    if let Some(roffset) = ridx.checked_sub(lidx + 1) {
                        let (lnums, rnums) = nums.split_at_mut(lidx + 1);
                        mem::swap(&mut lnums[lidx], &mut rnums[roffset]);
                    }
                } else {
                    lidx += 1;
                }
            }

            ridx
        };

        for idx in 0..n_pos {
            let value = nums[idx];
            let ref_idx = value.abs() as usize - 1;
            if ref_idx < n_pos {
                nums[ref_idx] = -nums[ref_idx].abs();
            }
        }

        (0..n_pos).find(|&idx| nums[idx] > 0).unwrap_or(n_pos) as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p41_test() {
        check(vec![1, 2, 0], 3);
        check(vec![3, 4, -1, 1], 2);
        check(vec![7, 8, 9, 11, 12], 1);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::first_missing_positive(nums), expect);
    }
}
