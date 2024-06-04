use crate::Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut s_iter = s.iter();

        for (idx, &tc) in t.iter().enumerate() {
            if !s_iter.any(|&sc| sc == tc) {
                return (t.len() - idx) as i32;
            }
        }

        0
    }
}
