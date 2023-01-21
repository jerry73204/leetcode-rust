use crate::Solution;
use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    pub fn word_pattern(pattern: String, text: String) -> bool {
        let mut chars = pattern.chars();
        let mut tokens = text.split_ascii_whitespace();
        let mut char_to_token: Vec<Option<&str>> = vec![None; 26];
        let mut token_to_char: HashMap<&str, char> = HashMap::new();

        loop {
            let (ch, token) = match (chars.next(), tokens.next()) {
                (Some(ch), Some(token)) => (ch, token),
                (None, None) => break,
                _ => return false,
            };

            let code = (ch as u32 - 'a' as u32) as usize;

            match char_to_token[code] {
                Some(prev) if prev == token => {}
                Some(_) => return false,
                None => {
                    char_to_token[code] = Some(token);
                }
            }

            match token_to_char.entry(token) {
                Entry::Occupied(entry) => {
                    if *entry.get() != ch {
                        return false;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(ch);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p290_test() {
        check("abba", "dog cat cat dog", true);
        check("abba", "dog cat cat fish", false);
        check("aaaa", "dog cat cat dog", false);
        check("aa", "dog dog dog", false);
        check("aaa", "dog dog", false);
        check("abba", "dog dog dog dog", false);
    }

    fn check(pattern: &str, text: &str, expect: bool) {
        assert_eq!(
            Solution::word_pattern(pattern.to_string(), text.to_string()),
            expect
        );
    }
}
