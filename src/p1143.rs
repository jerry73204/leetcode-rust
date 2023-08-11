use crate::Solution;
use std::cmp;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let chars1: Vec<char> = text1.chars().collect();
        let chars2: Vec<char> = text2.chars().collect();
        let mut tab = vec![vec![0; chars2.len() + 1]; chars1.len() + 1];

        for idx1 in 1..=chars1.len() {
            for idx2 in 1..=chars2.len() {
                let candidate = cmp::max(tab[idx1 - 1][idx2], tab[idx1][idx2 - 1]);

                tab[idx1][idx2] = if chars1[idx1 - 1] == chars2[idx2 - 1] {
                    cmp::max(tab[idx1 - 1][idx2 - 1] + 1, candidate)
                } else {
                    candidate
                };
            }
        }

        tab[chars1.len()][chars2.len()]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1143_test() {
        check("abcde", "ace", 3);
        check("abc", "abc", 3);
    }

    fn check(text1: &str, text2: &str, expect: i32) {
        assert_eq!(
            Solution::longest_common_subsequence(text1.to_string(), text2.to_string()),
            expect
        );
    }
}
