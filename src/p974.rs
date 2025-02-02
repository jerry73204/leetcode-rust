use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        #[derive(Debug)]
        struct State {
            acc: i32,
            remainder_counts: HashMap<i32, i32>,
        }

        let init_state = State {
            acc: 0,
            remainder_counts: {
                let mut map = HashMap::new();
                map.insert(0, 1);
                map
            },
        };

        let final_state = nums.into_iter().fold(init_state, |mut state, val| {
            state.acc = (state.acc + val) % k;
            if state.acc < 0 {
                state.acc += k;
            }

            state
                .remainder_counts
                .entry(state.acc)
                .and_modify(|cnt| {
                    *cnt += 1;
                })
                .or_insert(1);
            state
        });

        final_state
            .remainder_counts
            .into_values()
            .map(|count| count * (count - 1) / 2)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p974_test() {
        check(vec![4, 5, 0, -2, -3, 1], 5, 7);
        check(vec![0], 2, 1);
        check(vec![0, 1, 2], 3, 3);
    }

    fn check(nums: Vec<i32>, k: i32, expect: i32) {
        assert_eq!(Solution::subarrays_div_by_k(nums, k), expect);
    }
}
