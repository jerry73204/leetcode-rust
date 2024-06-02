use crate::Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let s = s.as_bytes();
        s.iter()
            .zip(&s[1..])
            .map(|(&a, &b)| (a as i32 - b as i32).abs())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3110_test() {
        check("hello", 13);
        check("zaz", 50);
    }

    fn check(s: &str, expect: i32) {
        assert_eq!(Solution::score_of_string(s.to_string()), expect);
    }
}
