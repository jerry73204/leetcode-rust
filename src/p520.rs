use crate::Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();

        let first = chars.next().unwrap();
        let second = chars.next();
        let second = match second {
            Some(ch) => ch,
            None => return true,
        };

        match (first.is_ascii_uppercase(), second.is_ascii_uppercase()) {
            (true, true) => chars.all(|ch| ch.is_ascii_uppercase()),
            (false, true) => false,
            (true | false, false) => chars.all(|ch| ch.is_ascii_lowercase()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p520_test() {
        check("USA", true);
        check("FlaG", false);
        check("Flag", true);
        check("nEw", false);
        check("A", true);
        check("a", true);
    }

    fn check(word: &str, expect: bool) {
        assert_eq!(Solution::detect_capital_use(word.to_string()), expect);
    }
}
