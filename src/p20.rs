use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut stack = Vec::with_capacity(bytes.len());

        for &ch in bytes {
            match ch {
                b'(' | b'[' | b'{' => {
                    stack.push(ch);
                }
                b')' => {
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}
