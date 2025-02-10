use crate::Solution;

impl Solution {
    pub fn clear_digits(input: String) -> String {
        let mut output = String::with_capacity(input.as_bytes().len());

        for ch in input.into_bytes() {
            if ch.is_ascii_digit() {
                output.pop();
            } else {
                output.push(ch as char);
            }
        }

        output
    }
}
