use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();

        if s.len() != t.len() {
            return false;
        }

        let mut char_count = [0usize; 26];
        for &ch in s {
            let idx = ch - b'a';
            char_count[idx as usize] += 1;
        }

        for &ch in t {
            let idx = ch - b'a';
            let count = &mut char_count[idx as usize];
            match count.checked_sub(1) {
                Some(new_count) => *count = new_count,
                None => return false,
            }
        }

        char_count.iter().all(|&cnt| cnt == 0)
    }
}
