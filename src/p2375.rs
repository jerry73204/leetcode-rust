//! # 2375. Construct Smallest Number From DI String
//! [Submission](https://leetcode.com/problems/construct-smallest-number-from-di-string/submissions/1547032872)

use crate::Solution;

impl Solution {
    pub fn smallest_number2(pattern: String) -> String {
        let pattern = pattern.as_bytes();

        let mut digits = vec![0u8; pattern.len() + 1];
        let mut marks = [false; 10];

        fn guess(
            digits: &mut [u8],
            marks: &mut [bool; 10],
            pattern: &[u8],
            prev_digit: u8,
        ) -> bool {
            let Some((digit, digits)) = digits.split_first_mut() else {
                return true;
            };
            let (&code, pattern) = pattern.split_first().unwrap();

            let range = match code {
                b'I' => (prev_digit + 1)..10,
                b'D' => 1..prev_digit,
                _ => unreachable!(),
            };

            for val in range {
                let mark = &mut marks[val as usize];
                if *mark {
                    continue;
                }

                *mark = true;
                *digit = val;
                if guess(digits, marks, pattern, val) {
                    return true;
                }
                marks[val as usize] = false;
            }

            false
        }

        {
            let (first, digits) = digits.split_first_mut().unwrap();

            for digit in 1u8..=9 {
                marks[digit as usize] = true;
                *first = digit;
                if guess(digits, &mut marks, &pattern, digit) {
                    break;
                }
                marks[digit as usize] = false;
            }
        }

        digits
            .into_iter()
            .map(|digit| (digit + b'0') as char)
            .collect()
    }
}
