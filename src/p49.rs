use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let num_words = strs.len();
        let mut len_groups = HashMap::new();

        strs.into_iter().for_each(|word| {
            let len = word.as_bytes().len();
            let group = len_groups
                .entry(len)
                .or_insert_with(|| Vec::with_capacity(num_words));
            group.push(word);
        });

        len_groups
            .into_iter()
            .flat_map(|(_, words)| {
                let mut anagram_groups = HashMap::new();
                let group_size = words.len();

                for word in words {
                    let counts = count_chars(&word);
                    let group = anagram_groups
                        .entry(counts)
                        .or_insert_with(|| Vec::with_capacity(group_size));
                    group.push(word);
                }

                anagram_groups
            })
            .map(|(_, group)| group)
            .collect()
    }
}

fn count_chars(word: &str) -> [usize; 26] {
    let mut count = [0; 26];
    for &ch in word.as_bytes() {
        let idx = ch - b'a';
        count[idx as usize] += 1;
    }
    count
}
