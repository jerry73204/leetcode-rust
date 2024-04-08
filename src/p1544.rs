use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn make_good(input: String) -> String {
        let mut lbuf: VecDeque<char> = input.chars().collect();
        let mut rbuf: VecDeque<char> = VecDeque::with_capacity(input.len());

        loop {
            if lbuf.is_empty() {
                return "".to_string();
            }

            std::mem::swap(&mut lbuf, &mut rbuf);
            lbuf.push_back(rbuf.pop_front().unwrap());

            let mut retry = false;

            while let Some(rchar) = rbuf.pop_front() {
                if let Some(&lchar) = lbuf.back() {
                    if is_bad_pair(lchar, rchar) {
                        lbuf.pop_back();
                        retry = true;
                    } else {
                        lbuf.push_back(rchar);
                    }
                } else {
                    lbuf.push_back(rchar);
                }
            }

            if !retry {
                break;
            }
        }

        lbuf.into_iter().collect()
    }
}

fn is_bad_pair(lc: char, rc: char) -> bool {
    (lc != rc) && (lc.to_ascii_uppercase() == rc || lc == rc.to_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1544_test() {
        check("leEeetcode", "leetcode");
        check("abBAcC", "");
        check("s", "s");
    }

    fn check(input: &str, expect: &str) {
        assert_eq!(Solution::make_good(input.to_string()), expect);
    }
}
