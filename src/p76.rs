use crate::Solution;

const N_CHARS: usize = 52;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let t_counts = count_chars(t_bytes);
        let mut window_counts = [0; N_CHARS];

        let mut n_ready_chars = t_counts.iter().filter(|&&count| count == 0).count();
        let mut start = 0;
        let mut end = 0;

        // This case happens only if t.len() == 0.
        if n_ready_chars == N_CHARS {
            return "".to_string();
        }

        // Extend the window until it contains all necessary chars.
        loop {
            // Extend the window by one step.
            let new_ch = s_bytes[end];
            let new_code = char_to_code(new_ch);
            end += 1;

            // Add one count for the newly added char.
            {
                let count = &mut window_counts[new_code];
                *count += 1;
                if *count == t_counts[new_code] {
                    n_ready_chars += 1;
                }
            }

            if n_ready_chars == N_CHARS {
                // If all chars ready, break the loop and go to the next
                // stage.
                break;
            } else if end == s_bytes.len() {
                // Otherwise, if there is no space to extend the
                // window, return an empty string.
                return "".to_string();
            }

            // Shrink the window.
            while start < end {
                let discard_ch = s_bytes[start];
                let discard_code = char_to_code(discard_ch);

                let count = &mut window_counts[discard_code];
                if *count > t_counts[discard_code] {
                    *count -= 1;
                    start += 1;
                } else {
                    break;
                }
            }
        }

        let mut min_range = start..end;

        while end < s_bytes.len() {
            // Extend the window by one step.
            let new_ch = s_bytes[end];
            let new_code = char_to_code(new_ch);
            end += 1;

            // Add one count for the newly added char.
            window_counts[new_code] += 1;

            // Shrink the window.
            while start < end {
                let discard_ch = s_bytes[start];
                let discard_code = char_to_code(discard_ch);

                let count = &mut window_counts[discard_code];
                if *count > t_counts[discard_code] {
                    *count -= 1;
                    start += 1;
                } else {
                    break;
                }
            }

            let curr_range = start..end;
            if curr_range.len() < min_range.len() {
                min_range = curr_range;
            }
        }

        s.chars()
            .skip(min_range.start)
            .take(min_range.len())
            .collect()
    }
}

fn count_chars(slice: &[u8]) -> [usize; N_CHARS] {
    let mut count = [0; N_CHARS];

    for &ch in slice {
        let code = char_to_code(ch);
        count[code] += 1
    }

    count
}

fn char_to_code(ch: u8) -> usize {
    (if (b'A'..=b'Z').contains(&ch) {
        ch - b'A'
    } else {
        // in a..z
        ch - b'a' + 26
    }) as usize
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p76_test() {
        check("ADOBECODEBANC", "ABC", "BANC");
        check("a", "a", "a");
        check("a", "aa", "");
        check("a", "", "");
        check("", "aa", "");
        check("xxxxxxaxxxxxxbxxxxaxxxxc", "abca", "axxxxxxbxxxxaxxxxc");
    }

    fn check(s: &str, t: &str, expect: &str) {
        assert_eq!(Solution::min_window(s.to_string(), t.to_string()), expect);
    }
}
