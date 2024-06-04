use crate::Solution;

impl Solution {
    pub fn longest_palindrome2(s: String) -> i32 {
        let char_to_code = |letter: u8| -> usize {
            let bit = 1 << 5;
            let cap = ((letter & bit) >> 5) * (b'Z' - b'A');
            let nth = (letter & !bit) - b'A';
            (cap + nth) as usize
        };

        let mut oddities = [false; 52];
        let mut len = 0;

        for &ch in s.as_bytes() {
            let code = char_to_code(ch);
            let oddity = &mut oddities[code];

            *oddity ^= true;

            if !*oddity {
                len += 2;
            }
        }

        if oddities.iter().any(|&odd| odd) {
            len += 1;
        }

        len
    }
}
