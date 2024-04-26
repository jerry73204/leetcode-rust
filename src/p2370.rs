use crate::Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut lens: [Option<usize>; 26] = [None; 26];

        s.as_bytes()
            .iter()
            .map(|&ch| {
                let curr = (ch - b'a') as usize;

                let start = curr.saturating_sub(k);
                let end = (curr + k).min(25);
                let len = lens[start..=end]
                    .iter()
                    .copied()
                    .flatten()
                    .max()
                    .unwrap_or(0)
                    + 1;
                lens[curr] = Some(len);
                len
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2370_test() {
        check("acfgbd", 2, 4);
        check("abcd", 3, 4);
        check("a", 3, 1);
        check("aaa", 0, 3);
        check("abaa", 0, 3);
    }

    fn check(s: &str, k: i32, expect: i32) {
        assert_eq!(Solution::longest_ideal_string(s.to_string(), k), expect);
    }
}
