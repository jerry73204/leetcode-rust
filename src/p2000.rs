use crate::Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let bytes = word.as_bytes();
        let ch = ch as u8;

        let prefix_len = match bytes.iter().enumerate().find(|&(_, &c)| c == ch) {
            Some((idx, _)) => idx + 1,
            None => return word,
        };

        let (prefix, suffix) = bytes.split_at(prefix_len);
        prefix
            .iter()
            .rev()
            .chain(suffix)
            .map(|&b| b as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2000_test() {
        check("abcdefd", 'd', "dcbaefd");
        check("xyxzxe", 'z', "zxyxxe");
        check("abcd", 'z', "abcd");
        check("z", 'z', "z");
    }

    fn check(word: &str, ch: char, expect: &str) {
        assert_eq!(Solution::reverse_prefix(word.to_string(), ch), expect);
    }
}
