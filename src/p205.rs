use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut forward = HashMap::new();
        let mut backward = HashMap::new();

        for (&lc, &rc) in s.iter().zip(t) {
            if let Some(other) = forward.insert(lc, rc) {
                if rc != other {
                    return false;
                }
            }

            if let Some(other) = backward.insert(rc, lc) {
                if lc != other {
                    return false;
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
    fn p205_test() {
        check("egg", "add", true);
        check("foo", "bar", false);
        check("paper", "title", true);
    }

    fn check(s: &str, t: &str, expect: bool) {
        assert_eq!(
            Solution::is_isomorphic(s.to_string(), t.to_string()),
            expect
        );
    }
}
