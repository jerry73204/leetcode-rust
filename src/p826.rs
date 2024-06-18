use crate::Solution;

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut indices: Vec<_> = (0..difficulty.len()).collect();
        indices.sort_unstable_by_key(|&ix| (difficulty[ix], -profit[ix]));

        let prefix_max: Vec<_> = indices
            .iter()
            .map(|&ix| profit[ix])
            .scan(i32::MIN, |max, profit| {
                *max = profit.max(*max);
                Some(*max)
            })
            .collect();

        if difficulty.len() < worker.len() {
            // Method 1
            worker
                .iter()
                .map(|cap| {
                    let nth = match indices.binary_search_by_key(cap, |&ix| difficulty[ix]) {
                        Ok(nth) => nth,
                        Err(nth) => match nth.checked_sub(1) {
                            Some(nth) => nth,
                            None => return 0,
                        },
                    };
                    prefix_max[nth]
                })
                .sum()
        } else {
            // Method 2
            worker.sort_unstable();

            let diff_iter = indices.iter().map(|&ix| difficulty[ix]);
            let prefix_max_iter = prefix_max.iter().copied();
            let mut iter = diff_iter.zip(prefix_max_iter).peekable();

            worker
                .into_iter()
                .scan(0, |max, cap| {
                    while let Some((_, profit)) = iter.next_if(|&(diff, _)| cap >= diff) {
                        *max = (*max).max(profit);
                    }
                    Some(*max)
                })
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p826_test() {
        check(
            vec![2, 4, 6, 8, 10],
            vec![10, 20, 30, 40, 50],
            vec![4, 5, 6, 7],
            100,
        );
        check(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25], 0);
    }

    fn check(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>, expect: i32) {
        assert_eq!(
            Solution::max_profit_assignment(difficulty, profit, worker),
            expect
        );
    }
}
