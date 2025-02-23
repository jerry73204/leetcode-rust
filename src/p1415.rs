//! # 1415. The k-th Lexicographical String of All Happy Strings of
//! Length n
//!
//! [Submission](https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/submissions/1553056025)

use crate::Solution;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let ix = (k - 1) as u32;

        // The first two bits of k from 0 to 3 decides the first
        // letter. Each of remaining bits the smaller or the greater
        // letter.
        //
        // For example, k = 0b101. The first two bits "10" implies
        // that the first latter is 'c'. The remaining bit "1"
        // determines that the second letter is 'b', which is the
        // greater one than 'a'.

        // Determine the first latter.
        let first = match ix / 2u32.pow(n as u32 - 1) {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => return "".to_string(),
        };

        // Determine remaining letters.
        let latter = (0..(n as u32 - 1))
            .rev()
            .map(|shift| 1 << shift)
            .map(|mask| ix & mask != 0)
            .scan(first, |prev, bit| {
                let curr = match (*prev, bit) {
                    ('a', false) => 'b',
                    ('a', true) => 'c',
                    ('b', false) => 'a',
                    ('b', true) => 'c',
                    ('c', false) => 'a',
                    ('c', true) => 'b',
                    _ => unreachable!(),
                };
                *prev = curr;
                Some(curr)
            });

        [first].into_iter().chain(latter).collect()
    }
}
