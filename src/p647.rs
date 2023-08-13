use crate::Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();

        // Count all 1-length palindromes.
        let mut count = bytes.len();

        // Count odd-length palindromes.
        for idx in 1..bytes.len() {
            let start_iter = (0..=(idx - 1)).rev();
            let end_iter = (idx + 2)..=bytes.len();

            for (start, end) in start_iter.zip(end_iter) {
                if bytes[start] == bytes[end - 1] {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        // Count even-length palindromes.
        for idx in 0..(bytes.len() - 1) {
            let start_iter = (0..=idx).rev();
            let end_iter = (idx + 2)..=bytes.len();

            for (start, end) in start_iter.zip(end_iter) {
                if bytes[start] == bytes[end - 1] {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        count as i32
    }
}
