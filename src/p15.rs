use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output = vec![];

        let uniq: Vec<(i32, usize)> = {
            nums.sort_unstable();
            let mut uniq = Vec::with_capacity(nums.len());
            let mut iter = nums.into_iter();
            let mut prev = iter.next().unwrap();
            let mut count = 1;

            for curr in iter {
                if prev == curr {
                    count += 1;
                } else {
                    uniq.push((prev, count));
                    prev = curr;
                    count = 1;
                }
            }

            uniq.push((prev, count));
            uniq
        };

        // Case: 3 same numbers
        {
            let idx = uniq.binary_search_by_key(&0, |(val, _)| *val);
            if let Ok(idx) = idx {
                let (_, count) = uniq[idx];
                if count >= 3 {
                    output.push(vec![0, 0, 0]);
                }
            }
        }

        // Case: 2 same numbers and 1 distinct number
        for &(lval, lcount) in &uniq {
            if lcount < 2 || lval == 0 {
                continue;
            }
            let rval = 0 - 2 * lval;

            let index = uniq.binary_search_by_key(&rval, |(val, _)| *val);
            if index.is_ok() {
                output.push(vec![lval, lval, rval]);
            }
        }

        // Case: 3 distinct numbers
        for (lidx, &(lval, _)) in uniq.iter().enumerate() {
            for (moffset, &(mval, _)) in uniq[(lidx + 1)..].iter().enumerate() {
                let midx = lidx + 1 + moffset;
                let rval = 0 - lval - mval;

                if rval <= mval {
                    continue;
                }

                let roffset = uniq[(midx + 1)..].binary_search_by_key(&rval, |(val, _)| *val);
                if roffset.is_ok() {
                    output.push(vec![lval, mval, rval]);
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p15_test() {
        check(
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        );
        check(vec![0, 1, 1], vec![]);
        check(vec![0, 0, 0], vec![vec![0, 0, 0]]);
        check(
            vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6],
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2],
            ],
        );
    }

    fn check(nums: Vec<i32>, mut expect: Vec<Vec<i32>>) {
        let mut answer = Solution::three_sum(nums);

        let sort = |slice: &mut [Vec<i32>]| {
            for nums in &mut *slice {
                nums.sort_unstable();
            }
            slice.sort_unstable();
        };

        sort(&mut answer);
        sort(&mut expect);

        assert_eq!(answer, expect);
    }
}
