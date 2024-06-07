use crate::Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut freq1 = count_letters(&word1);
        let mut freq2 = count_letters(&word2);

        let mut n_letters = 0;
        for (&cnt1, &cnt2) in freq1.iter().zip(&freq2) {
            if (cnt1 > 0) ^ (cnt2 > 0) {
                return false;
            }
            n_letters += 1;
        }

        freq1.sort_unstable_by_key(|&val| Reverse(val));
        freq2.sort_unstable_by_key(|&val| Reverse(val));

        freq1[0..n_letters] == freq2[0..n_letters]
    }
}

fn count_letters(input: &str) -> [i32; 26] {
    let mut counts = [0; 26];

    for &ch in input.as_bytes() {
        let ix = (ch - b'a') as usize;
        counts[ix] += 1;
    }

    counts
}
