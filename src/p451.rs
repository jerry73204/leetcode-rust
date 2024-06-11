use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let i2a: Vec<_> = (b'A'..=b'Z')
            .chain(b'a'..=b'z')
            .chain(b'0'..=b'9')
            .collect();
        let a2i: HashMap<u8, usize> = i2a
            .iter()
            .enumerate()
            .map(|(code, &letter)| (letter, code))
            .collect();

        let mut counts = [0; 62];

        for ch in s.as_bytes() {
            let code = a2i[&ch];
            counts[code] += 1;
        }

        let mut indices: Vec<_> = (0..counts.len()).collect();
        indices.sort_unstable_by_key(|&idx| -(counts[idx] as isize));

        let len = match indices.iter().enumerate().find(|(_, freq)| **freq == 0) {
            Some((idx, _)) => idx,
            None => indices.len(),
        };

        indices[0..len]
            .iter()
            .flat_map(|&code| {
                let freq = counts[code];
                let letter = i2a[code] as char;
                std::iter::repeat(letter).take(freq)
            })
            .collect()
    }
}
