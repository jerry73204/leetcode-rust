use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut all_win = HashSet::new();
        let mut one_lose = HashSet::new();
        let mut more_loses = HashSet::new();

        for m in matches {
            let winner = m[0];
            let loser = m[1];

            if !one_lose.contains(&winner) && !more_loses.contains(&winner) {
                all_win.insert(winner);
            }

            if all_win.contains(&loser) {
                all_win.remove(&loser);
                one_lose.insert(loser);
            } else if one_lose.contains(&loser) {
                one_lose.remove(&loser);
                more_loses.insert(loser);
            } else if !more_loses.contains(&loser) {
                one_lose.insert(loser);
            }
        }

        let mut all_win: Vec<_> = all_win.into_iter().collect();
        let mut one_lose: Vec<_> = one_lose.into_iter().collect();

        all_win.sort_unstable();
        one_lose.sort_unstable();

        vec![all_win, one_lose]
    }
}
