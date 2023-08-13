use crate::Solution;

impl Solution {
    // Rename to `is_palindrome`
    pub fn is_palindrome_p125(s: String) -> bool {
        let text: Vec<_> = s
            .as_bytes()
            .iter()
            .filter_map(|&ch| {
                if (b'a'..=b'z').contains(&ch) || (b'0'..=b'9').contains(&ch) {
                    Some(ch)
                } else if (b'A'..=b'Z').contains(&ch) {
                    Some(ch - b'A' + b'a')
                } else {
                    None
                }
            })
            .collect();

        let half_len = text.len() / 2;

        text.iter()
            .zip(text.iter().rev())
            .take(half_len)
            .all(|(&lch, &rch)| lch == rch)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p125_test() {
        check("0P", false);
    }

    fn check(s: &str, expect: bool) {
        assert_eq!(Solution::is_palindrome_p125(s.to_string()), expect);
    }
}
