use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut max_len = 1;
        let mut max_range = 0..1;

        for idx in 0..bytes.len() {
            // Odd
            {
                let min_pad = (max_len - 1) / 2 + 1;
                if let Some(lbound) = idx.checked_sub(min_pad) {
                    let rbound = idx + 1 + min_pad;

                    let start_iter = (0..=lbound).rev();
                    let end_iter = rbound..=bytes.len();

                    for (start, end) in start_iter.zip(end_iter) {
                        let range = start..end;

                        if is_palindrome(&bytes[range.clone()]) {
                            let len = end - start;
                            max_len = len;
                            max_range = range;
                        } else {
                            break;
                        }
                    }
                }
            }

            // Even
            {
                let min_pad = max_len / 2 + 1;
                if let Some(lbound) = (idx + 1).checked_sub(min_pad) {
                    let rbound = idx + 1 + min_pad;

                    let start_iter = (0..=lbound).rev();
                    let end_iter = rbound..=bytes.len();

                    for (start, end) in start_iter.zip(end_iter) {
                        let range = start..end;

                        if is_palindrome(&bytes[range.clone()]) {
                            let len = end - start;
                            max_len = len;
                            max_range = range;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        s.chars()
            .skip(max_range.start)
            .take(max_range.len())
            .collect()
    }
}

fn is_palindrome(slice: &[u8]) -> bool {
    let half_len = slice.len() / 2;
    slice
        .iter()
        .zip(slice.iter().rev())
        .take(half_len)
        .all(|(&lch, &rch)| lch == rch)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p5_test() {
        check("a", "a");
        check("bb", "bb");
        check("aba", "aba");
        check("abc", "a");
    }

    fn check(s: &str, expect: &str) {
        assert_eq!(Solution::longest_palindrome(s.to_string()), expect);
    }
}
