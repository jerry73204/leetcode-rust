use crate::Solution;
use std::{
    cmp::{self, Ordering},
    collections::BinaryHeap,
};

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let to_usize_vec =
            |s: &str| -> Vec<usize> { s.as_bytes().iter().map(|&c| (c - b'a') as usize).collect() };

        let ring = to_usize_vec(&ring);
        let key = to_usize_vec(&key);

        let letter_lookup =
            ring.iter()
                .copied()
                .enumerate()
                .fold(vec![vec![]; 26], |mut lookup, (ix, ch)| {
                    lookup[ch].push(ix);
                    lookup
                });

        #[derive(Debug)]
        struct State {
            cost: i32,
            ring_ix: usize,
            key_ix: usize,
        }

        impl PartialEq for State {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Eq for State {}

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                self.cost.cmp(&other.cost).reverse()
            }
        }

        let count_steps = |curr: usize, next: usize| {
            let min = cmp::min(curr, next);
            let max = cmp::max(curr, next);
            let steps1 = max - min;
            let steps2 = ring.len() - steps1;
            cmp::min(steps1, steps2) as i32 + 1
        };

        let mut dist_lookup = vec![vec![i32::MAX; key.len() + 1]; ring.len()];
        let mut frontiers = BinaryHeap::new();

        let init = State {
            cost: 0,
            ring_ix: 0,
            key_ix: 0,
        };
        frontiers.push(init);

        loop {
            let curr = frontiers.pop().unwrap();
            let State {
                cost,
                ring_ix,
                key_ix,
            } = curr;

            let key_ch = match key.get(key_ix) {
                Some(&ch) => ch,
                None => break cost,
            };

            let curr_dist = dist_lookup[ring_ix][key_ix];

            if cost > curr_dist {
                continue;
            }

            for &next_ring_ix in &letter_lookup[key_ch] {
                let next_key_ix = key_ix + 1;
                let next_cost = cost + count_steps(ring_ix, next_ring_ix);
                let next_dist = &mut dist_lookup[next_ring_ix][next_key_ix];

                if next_cost < *next_dist {
                    *next_dist = next_cost;
                    let next = State {
                        cost: next_cost,
                        ring_ix: next_ring_ix,
                        key_ix: next_key_ix,
                    };

                    frontiers.push(next);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p514_test() {
        check("godding", "gd", 4);
        check("godding", "godding", 13);
        check("aaa", "a", 1);
        check("a", "aaa", 3);
        check("ba", "aaa", 4);
        check("bbacc", "aaa", 5);
        check("abab", "baba", 8);
    }

    fn check(ring: &str, key: &str, expect: i32) {
        assert_eq!(
            Solution::find_rotate_steps(ring.to_string(), key.to_string()),
            expect
        );
    }
}
