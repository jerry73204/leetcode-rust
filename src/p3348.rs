//! # 3348. Smallest Divisible Digit Product II
//! [submission](https://leetcode.com/problems/smallest-divisible-digit-product-ii/submissions/1538748904)

use crate::Solution;
use std::{
    iter::repeat,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

// Power vec for digits 0~9
const DIGIT_POWER_VECS: [PowerVec; 10] = [
    PowerVec([0, 0, 0, 0]), // 0
    PowerVec([0, 0, 0, 0]), // 1
    PowerVec([1, 0, 0, 0]), // 2
    PowerVec([0, 1, 0, 0]), // 3
    PowerVec([2, 0, 0, 0]), // 4
    PowerVec([0, 0, 1, 0]), // 5
    PowerVec([1, 1, 0, 0]), // 6
    PowerVec([0, 0, 0, 1]), // 7
    PowerVec([3, 0, 0, 0]), // 8
    PowerVec([0, 2, 0, 0]), // 9
];

impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        // Factorize t to a power vec
        let t_power_vec: PowerVec = {
            let mut powers = [0; 4];
            let mut r = t;

            for (prime, power) in PowerVec::PRIMES.iter().zip(powers.iter_mut()) {
                while r % prime == 0 {
                    r /= prime;
                    *power += 1;
                }
            }

            if r != 1 {
                return "-1".to_string();
            }

            powers.into()
        };

        // Extract the prefixing digits of num that ends at the first
        // zero or at the last digit.
        let prefix_end = bytes2digits(num.as_bytes())
            .enumerate()
            .find(|(_, digit)| *digit == 0)
            .map(|(idx, _)| idx);
        let prefix_bytes = match prefix_end {
            Some(prefix_end) => &num.as_bytes()[0..=prefix_end],
            None => num.as_bytes(),
        };
        let prefix_power_vec = sum_power_vec_of_digits(bytes2digits(prefix_bytes));
        let mut diff_power_vec = &prefix_power_vec - &t_power_vec;

        {
            let &last_byte = prefix_bytes.last().unwrap();
            if byte2digit(last_byte) != 0 && !diff_power_vec.contains_neg() {
                return num;
            }
        }

        for (idx, low_digit) in bytes2digits(prefix_bytes).enumerate().rev() {
            let suffix_len = num.as_bytes().len() - idx - 1;
            diff_power_vec -= &DIGIT_POWER_VECS[low_digit];

            for replace_digit in (low_digit + 1)..=9 {
                let diff2_power_vec = &diff_power_vec + &DIGIT_POWER_VECS[replace_digit];
                let Some(digit_counts) =
                    solve_minimum_power_vec_cover(&diff2_power_vec, suffix_len)
                else {
                    continue;
                };

                let output = {
                    let mut output = String::with_capacity(num.len());
                    let prefix_bytes = &num.as_bytes()[0..idx];
                    output.extend(prefix_bytes.iter().map(|&b| b as char));
                    output.push(digit2char(replace_digit));

                    for (&count, digit) in digit_counts[1..].iter().zip('1'..) {
                        output.extend(repeat(digit).take(count as usize))
                    }

                    output
                };
                return output;
            }
        }

        {
            let digit_counts = solve_minimum_power_vec_cover(&diff_power_vec, usize::MAX).unwrap();

            let mut output = String::with_capacity(num.len());

            // In the case that the n. of digits equal to the original
            // lenght, add an extra '1' prefix.
            let total_count: usize = digit_counts[2..].iter().sum();

            if let Some(pad_count) = (num.as_bytes().len() + 1).checked_sub(total_count) {
                output.extend(repeat('1').take(pad_count));
            }

            for (&count, digit) in digit_counts[2..].iter().zip('2'..) {
                output.extend(repeat(digit).take(count as usize))
            }

            output
        }
    }
}

fn solve_minimum_power_vec_cover(diff: &PowerVec, max_places: usize) -> Option<[usize; 10]> {
    let mut remain_places = max_places as u32;
    let PowerVec([diff2, diff3, diff5, diff7]) = *diff;

    let pow7 = if diff7 < 0 {
        let pow7 = -diff7 as usize;
        remain_places = remain_places.checked_sub(pow7 as u32)?;
        pow7
    } else {
        0
    };

    let pow5 = if diff5 < 0 {
        let pow5 = -diff5 as usize;
        remain_places = remain_places.checked_sub(pow5 as u32)?;
        pow5
    } else {
        0
    };

    let mut pow2 = (-diff2).max(0) as usize;
    let mut pow3 = (-diff3).max(0) as usize;

    let pow8 = pow2 / 3;
    pow2 -= pow8 * 3;
    remain_places = remain_places.checked_sub(pow8 as u32)?;

    let pow9 = pow3 / 2;
    pow3 -= pow9 * 2;
    remain_places = remain_places.checked_sub(pow9 as u32)?;

    let mut pow4 = pow2 / 2;
    pow2 -= pow4 * 2;

    let pow6: usize = match (pow2, pow3) {
        (0, 0) | (1, 0) => 0,
        (0, 1) => {
            if pow4 > 0 {
                pow3 -= 1;
                pow4 -= 1;
                pow2 += 1;
                1
            } else {
                0
            }
        }
        (1, 1) => {
            pow2 -= 1;
            pow3 -= 1;
            1
        }
        _ => unreachable!(),
    };

    remain_places = remain_places.checked_sub(pow2 as u32)?;
    remain_places = remain_places.checked_sub(pow3 as u32)?;
    remain_places = remain_places.checked_sub(pow4 as u32)?;
    remain_places = remain_places.checked_sub(pow6 as u32)?;

    let pow1 = remain_places as usize;

    let counts = [0usize, pow1, pow2, pow3, pow4, pow5, pow6, pow7, pow8, pow9];
    Some(counts)
}

fn sum_power_vec_of_digits(digits: impl IntoIterator<Item = usize>) -> PowerVec {
    let mut powers = PowerVec([0; 4]);

    for digit in digits {
        powers += &DIGIT_POWER_VECS[digit];
    }

    powers
}

fn bytes2digits(
    bytes: &[u8],
) -> impl Iterator<Item = usize> + DoubleEndedIterator + ExactSizeIterator + '_ {
    bytes.iter().copied().map(byte2digit)
}

fn byte2digit(ch: u8) -> usize {
    (ch - b'0') as usize
}

fn digit2char(digit: usize) -> char {
    (digit as u8 + b'0') as char
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PowerVec(pub [i32; 4]);

impl PowerVec {
    // Prime numbers corresponding to each slot of power vec.
    const PRIMES: [i64; 4] = [2, 3, 5, 7];

    fn contains_neg(&self) -> bool {
        self.0.iter().any(|v| v.is_negative())
    }
}

impl From<[i32; 4]> for PowerVec {
    fn from(powers: [i32; 4]) -> Self {
        Self(powers)
    }
}

impl Add for &PowerVec {
    type Output = PowerVec;

    fn add(self, rhs: Self) -> Self::Output {
        let mut powers = [0; 4];
        for ((&a, &b), c) in self.0.iter().zip(&rhs.0).zip(&mut powers) {
            *c = a + b;
        }
        PowerVec(powers)
    }
}

impl AddAssign<&PowerVec> for PowerVec {
    fn add_assign(&mut self, rhs: &PowerVec) {
        for (a, b) in self.0.iter_mut().zip(&rhs.0) {
            *a += b;
        }
    }
}

impl Sub for &PowerVec {
    type Output = PowerVec;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut powers = [0; 4];
        for ((&a, &b), c) in self.0.iter().zip(&rhs.0).zip(&mut powers) {
            *c = a - b;
        }
        PowerVec(powers)
    }
}

impl Neg for &PowerVec {
    type Output = PowerVec;

    fn neg(self) -> Self::Output {
        let mut powers = [0; 4];
        for (&a, b) in self.0.iter().zip(&mut powers) {
            *b = -a;
        }
        PowerVec(powers)
    }
}

impl SubAssign<&PowerVec> for PowerVec {
    fn sub_assign(&mut self, rhs: &PowerVec) {
        for (a, b) in self.0.iter_mut().zip(&rhs.0) {
            *a -= b;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3348_test() {
        check("1234", 256, "1488");
        check("12355", 50, "12355");
        check("11111", 26, "-1");
        check("30", 9, "33");
        check("1", 9, "9");
        check("11", 9, "19");
        check("1", 65536, "288888");
        check("78", 42, "167");
        check(
            "999999999999999999999999999",
            99995938560000,
            "1111111111125555677888999999",
        );
    }

    fn check(num: &str, t: i64, expect: &str) {
        assert_eq!(Solution::smallest_number(num.to_string(), t), expect);
    }
}
