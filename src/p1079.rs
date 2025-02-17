use crate::Solution;
use std::cmp::min;

const FACTORIAL: [i32; 9] = [1, 1, 2, 6, 24, 120, 720, 5040, 40240];

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts = [0; 26];

        for &letter in tiles.as_bytes() {
            // *counts.entry(letter).or_insert(0) += 1;
            let idx = (letter - b'A') as usize;
            counts[idx] += 1;
        }

        let mut coefs: [i32; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
        let mut max_power = 0;

        for count in counts.into_iter().filter(|&c| c != 0) {
            let mut new_coefs: [i32; 8] = [0; 8];
            max_power += count;

            for (power, new_coef) in (0..=max_power).zip(&mut new_coefs) {
                for a in 0..=min(count, power) {
                    let b = power - a;
                    *new_coef += comb(a + b, a) * coefs[b as usize];
                }
            }

            coefs = new_coefs;
        }

        let sum: i32 = coefs[0..=(max_power as usize)].iter().sum();
        sum - 1
    }
}

fn comb(n: i32, r: i32) -> i32 {
    let n = n as usize;
    let r = r as usize;
    FACTORIAL[n] / FACTORIAL[r] / FACTORIAL[n - r]
}
