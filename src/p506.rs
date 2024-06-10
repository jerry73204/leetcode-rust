use crate::Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut indices: Vec<usize> = (0..score.len()).collect();
        indices.sort_unstable_by_key(|&idx| Reverse(score[idx]));

        let mut answer = vec![String::new(); score.len()];

        for (idx, rank) in indices.into_iter().zip(1..) {
            let award = match rank {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                _ => rank.to_string(),
            };
            answer[idx] = award;
        }

        answer
    }
}
