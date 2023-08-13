use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<_> = s.chars().collect();
        let char_set: HashSet<char> = chars.iter().cloned().collect();
        let max_replace = k as usize;
        let mut max_len = 0;

        for target_ch in char_set {
            let mut start = 0;
            let mut replace = 0;

            for (&curr_ch, end) in chars.iter().zip(1..) {
                if target_ch != curr_ch {
                    if replace == max_replace {
                        while start < end {
                            let is_matched = chars[start] == target_ch;
                            start += 1;
                            if !is_matched {
                                break;
                            }
                        }
                    } else {
                        replace += 1;
                    }
                }

                let len = end - start;
                max_len = max_len.max(len);
            }
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p424_test() {
        check("ABAB", 2, 4);
        check("AABABBA", 1, 4);
        check("AABABBA", 0, 2);
        check("", 4, 0);
        check("", 0, 0);
    }

    fn check(s: &str, k: i32, expect: i32) {
        assert_eq!(Solution::character_replacement(s.to_string(), k), expect);
    }
}
