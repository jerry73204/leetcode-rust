use crate::Solution;
use std::cmp;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let acc_inc: Vec<i32> = {
            let mut acc_inc = Vec::with_capacity(ratings.len());
            acc_inc.push(0);

            let iter = ratings
                .iter()
                .zip(&ratings[1..])
                .scan(0, |acc, (&lhs, &rhs)| {
                    Some(if lhs < rhs {
                        *acc += 1;
                        *acc
                    } else {
                        *acc = 0;
                        0
                    })
                });
            acc_inc.extend(iter);
            acc_inc
        };
        let acc_dec: Vec<i32> = {
            let mut acc_dec = Vec::with_capacity(ratings.len());
            acc_dec.push(0);

            let iter = ratings
                .iter()
                .zip(&ratings[1..])
                .rev()
                .scan(0, |acc, (&lhs, &rhs)| {
                    Some(if lhs > rhs {
                        *acc += 1;
                        *acc
                    } else {
                        *acc = 0;
                        0
                    })
                });
            acc_dec.extend(iter);
            acc_dec
        };

        acc_inc
            .into_iter()
            .zip(acc_dec.into_iter().rev())
            .map(|(left_inc, right_inc)| cmp::max(left_inc, right_inc) + 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p135_test() {
        check(vec![1, 0, 2], 5);
        check(vec![1, 2, 2], 4);
    }

    fn check(ratings: Vec<i32>, expect: i32) {
        assert_eq!(Solution::candy(ratings), expect);
    }
}
