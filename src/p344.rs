use crate::Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}
