//! # 2698. Find the Punishment Number of an Integer
//! [Submission](https://leetcode.com/problems/find-the-punishment-number-of-an-integer/submissions/1546305180)
use crate::Solution;

const PUNISHMENT_NUMBERS: &[i32] = &[
    1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756, 792,
    909, 918, 945, 964, 990, 991, 999, 1000,
];
const SQUARES: &[i32] = &[
    1, 81, 100, 1296, 2025, 3025, 6724, 8281, 9801, 10000, 55225, 88209, 136161, 136900, 143641,
    171396, 431649, 455625, 494209, 571536, 627264, 826281, 842724, 893025, 929296, 980100, 982081,
    998001, 1000000,
];

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let idx = match PUNISHMENT_NUMBERS.binary_search(&n) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };
        SQUARES[0..=idx].iter().sum()
    }
}

/// The function checks if n is a punishment number or not.
#[allow(unused)]
fn is_punishment_number(n: i32) -> bool {
    let n2 = n * n;

    let digits: Vec<i32> = n2
        .to_string()
        .into_bytes()
        .into_iter()
        .map(|ch| (ch - b'0') as i32)
        .collect();

    fn check(n: i32, digits: &[i32], sum: i32) -> bool {
        if digits.is_empty() && n == sum {
            return true;
        }

        let mut val = 0;

        for (idx, digit) in digits.iter().enumerate() {
            val = val * 10 + digit;
            let next_sum = sum + val;
            if next_sum > n {
                break;
            }

            if check(n, &digits[(idx + 1)..], sum + val) {
                return true;
            }
        }

        false
    }

    check(n, &digits, 0)
}
