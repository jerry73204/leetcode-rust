use crate::Solution;
use std::{collections::VecDeque, iter};

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        words.sort_by_cached_key(|word| word.as_bytes().len());

        // Group words by word lengths
        let mut groups = iter::repeat(()).scan(words.as_slice(), |slice: &mut _, ()| {
            let first = slice.first()?;
            let word_len = first.as_bytes().len();
            let end_index = slice
                .iter()
                .enumerate()
                .find(|(_, word)| word.as_bytes().len() > word_len)
                .map(|(index, _)| index);
            let end_index = if let Some(end_index) = end_index {
                end_index
            } else {
                slice.len()
            };

            let (group, remain) = slice.split_at(end_index);
            *slice = remain;

            Some(group)
        });

        // Build a trie
        let mut nodes = vec![Node::default()];

        // Insert the first group, which are words with smallest length
        {
            let group = groups.next().unwrap();
            for word in group {
                insert(&mut nodes, word.as_bytes());
            }
        }

        // Scan later groups
        let mut output = vec![];

        // For each group, find concatenated words in the group. Then,
        // insert the words to the trie.
        for group in groups {
            for word in group {
                if check(&nodes, word.as_bytes()) {
                    output.push(word.to_string());
                }
            }

            for word in group {
                insert(&mut nodes, word.as_bytes());
            }
        }

        output
    }
}

fn insert(nodes: &mut Vec<Node>, word: &[u8]) {
    let end_index =
        word.iter()
            .map(|&symbol| (symbol - b'a') as usize)
            .fold(0, |curr_index, symbol_index| {
                let node = &mut nodes[curr_index];

                if let Some(child_index) = node.children[symbol_index] {
                    child_index
                } else {
                    nodes.push(Node::default());
                    let child_index = nodes.len() - 1;
                    nodes[curr_index].children[symbol_index] = Some(child_index);
                    child_index
                }
            });

    let end_node = &mut nodes[end_index];
    end_node.is_end = true;
}

fn check(nodes: &[Node], word: &[u8]) -> bool {
    let mut candidates = VecDeque::new();
    candidates.push_back(0);

    for symbol in word {
        let symbol_index = (symbol - b'a') as usize;
        let mut has_fallback = false;

        for _ in 0..candidates.len() {
            let candidate_index = candidates.pop_front().unwrap();
            let candidate = &nodes[candidate_index];
            let child_index = candidate.children[symbol_index];

            if let Some(child_index) = child_index {
                let child = &nodes[child_index];
                if child.is_end {
                    has_fallback = true;
                }
                candidates.push_back(child_index);
            }
        }

        if has_fallback {
            candidates.push_front(0);
        }

        if candidates.is_empty() {
            return false;
        }
    }

    candidates.into_iter().any(|candidate_index| {
        let candidate = &nodes[candidate_index];
        candidate.is_end
    })
}

#[derive(Debug, Default)]
struct Node {
    pub children: [Option<usize>; 26],
    pub is_end: bool,
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::HashSet;

    #[test]
    fn p472_test() {
        check(
            &[
                "cat",
                "cats",
                "catsdogcats",
                "dog",
                "dogcatsdog",
                "hippopotamuses",
                "rat",
                "ratcatdogcat",
            ],
            &["catsdogcats", "dogcatsdog", "ratcatdogcat"],
        );
        check(&["cat", "dog", "catdog"], &["catdog"]);
        check(&["aaa", "aaaaaa"], &["aaaaaa"]);
        check(
            &["abc", "def", "ghi", "abcdef", "abcdefghi"],
            &["abcdef", "abcdefghi"],
        );
    }

    fn check(words: &[&str], expect: &[&str]) {
        let words: Vec<String> = words.iter().map(|word| word.to_string()).collect();
        let answer: HashSet<String> = Solution::find_all_concatenated_words_in_a_dict(words)
            .into_iter()
            .collect();
        let expect: HashSet<String> = expect.iter().map(|word| word.to_string()).collect();
        assert_eq!(answer, expect);
    }
}
