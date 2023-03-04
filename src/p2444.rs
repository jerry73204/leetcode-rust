use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let nums_len = nums.len();

        if min_k == max_k {
            let mut iter = nums.into_iter().enumerate().peekable();
            let mut count = 0;

            while let Some((begin, first)) = iter.next() {
                if first != min_k {
                    continue;
                }

                while iter.next_if(|&(_, val)| val == min_k).is_some() {}

                let end = match iter.peek() {
                    Some(&(index, _)) => index,
                    None => nums_len,
                };

                let len = end - begin;
                count += len + len * (len - 1) / 2;
            }

            return count as i64;
        }

        let mut iter = nums.into_iter().enumerate().peekable();

        let mut max_runs = VecDeque::with_capacity(nums_len);
        let mut min_runs = VecDeque::with_capacity(nums_len);
        let mut unused_runs = VecDeque::with_capacity(nums_len);

        while let Some((begin, first)) = iter.next() {
            if first < min_k || first > max_k {
                let mut end = begin + 1;
                while iter
                    .next_if(|&(_, val)| val < min_k || val > max_k)
                    .is_some()
                {
                    end += 1;
                }

                unused_runs.push_back(begin..end);
            } else if first == min_k {
                min_runs.push_back(begin..(begin + 1));
            } else if first == max_k {
                max_runs.push_back(begin..(begin + 1));
            } else {
                while iter
                    .next_if(|&(_, val)| val > min_k && val < max_k)
                    .is_some()
                {}

                let (index, val) = match iter.peek() {
                    Some(tuple) => *tuple,
                    None => break,
                };

                if val == min_k {
                    iter.next().unwrap();
                    min_runs.push_back(begin..(index + 1));
                } else if val == max_k {
                    iter.next().unwrap();
                    max_runs.push_back(begin..(index + 1));
                } else {
                    // Merge this run to the most recent run
                }
            }
        }

        let mut count = 0;

        loop {
            let unused_run = unused_runs.front();
            let min_run = min_runs.front();
            let max_run = max_runs.front();

            let (min_run, max_run) = match (min_run, max_run) {
                (Some(min_run), Some(max_run)) => (min_run, max_run),
                (_, _) => break,
            };

            if let Some(unused_run) = unused_run {
                let preceed_min_run = unused_run.end <= min_run.start;
                let preceed_max_run = unused_run.end <= max_run.start;

                match (preceed_min_run, preceed_max_run) {
                    (true, true) => {
                        unused_runs.pop_front();
                        continue;
                    }
                    (true, false) => {
                        max_runs.pop_front();
                        continue;
                    }
                    (false, true) => {
                        min_runs.pop_front();
                        continue;
                    }
                    (false, false) => {}
                }
            }

            let (lbound_range, rbound_start) = if min_run.start < max_run.start {
                let leftmost_run = min_runs.pop_front().unwrap();
                (leftmost_run, max_run.end)
            } else {
                let leftmost_run = max_runs.pop_front().unwrap();
                (leftmost_run, min_run.end)
            };
            let rbound_end = match unused_run {
                Some(range) => range.start,
                None => nums_len,
            };

            count += lbound_range.len() * (rbound_end - rbound_start + 1);
        }

        count as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2444_test() {
        check(vec![1, 3, 5, 2, 7, 5], 1, 5, 2);
        check(vec![1, 1, 1, 1], 1, 1, 10);
        check(vec![2, 1, 1, 1, -1, 1, 3], 1, 1, 7);
        check(vec![-1, 5, 8, 1, 7], 1, 5, 0);
        check(vec![], 1, 5, 0);
        check(vec![], 1, 1, 0);
        check(vec![2, 2, 2], 2, 2, 6);
    }

    fn check(vals: Vec<i32>, min_k: i32, max_k: i32, expect: i64) {
        assert_eq!(Solution::count_subarrays(vals, min_k, max_k), expect);
    }
}
