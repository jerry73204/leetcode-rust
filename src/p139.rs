use crate::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let text = s.as_bytes();
        let mut tab = vec![false; text.len() + 1];
        tab[0] = true;

        for end in 1..=text.len() {
            for word in &word_dict {
                let word = word.as_bytes();

                let start = end.checked_sub(word.len());
                if let Some(start) = start {
                    if &text[start..end] == word && tab[start] {
                        tab[end] = true;
                        break;
                    }
                }
            }
        }

        *tab.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p139_test() {
        check("leetcode", &["leet", "code"], true);
        check("applepenapple", &["apple", "pen"], true);
        check("catsandog", &["cats", "dog", "sand", "and", "cat"], false);
    }

    fn check(s: &str, word_dict: &[&str], expect: bool) {
        let word_dict: Vec<_> = word_dict.iter().map(|w| w.to_string()).collect();
        assert_eq!(Solution::word_break(s.to_string(), word_dict), expect);
    }
}
