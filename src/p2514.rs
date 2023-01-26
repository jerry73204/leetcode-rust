use crate::Solution;
use std::{cmp::Ordering, iter};

const MODULO: i64 = 1000000007;

impl Solution {
    pub fn count_anagrams(input: String) -> i32 {
        let words: Vec<&str> = input.split_ascii_whitespace().collect();
        let max_word_len = words.iter().map(|word| word.len()).max().unwrap();

        let factorials: Vec<i64> = {
            let iter = (1..=(max_word_len as i64)).scan(1, |fac, val| {
                *fac = (*fac * val) % MODULO;
                Some(*fac)
            });
            iter::once(1).chain(iter).collect()
        };
        // let factorial_inverses: Vec<Option<i64>> = vec![None; max_word_len + 1];

        let count = words
            .into_iter()
            .map(|word| {
                let (n_chars, duplicated_char_counts) = count_duplicated_chars(word);
                let numerator = factorials[n_chars as usize];
                let denominator = duplicated_char_counts
                    .into_iter()
                    .map(|count| factorials[count as usize])
                    .fold(1, |prod, fac| (prod * fac) % MODULO);

                (numerator * modulo_inverse(denominator)) % MODULO
            })
            .fold(1, |prod, count| (prod * count) % MODULO);

        count as i32
    }
}

fn count_duplicated_chars(word: &str) -> (i64, Vec<i64>) {
    let mut counts = [0; 26];
    word.chars().for_each(|ch| {
        let index = ch as u32 - 'a' as u32;
        counts[index as usize] += 1;
    });

    let mut total_count = 0;
    let mut char_counts = Vec::with_capacity(26);

    for count in counts {
        total_count += count;

        if count > 1 {
            char_counts.push(count);
        }
    }

    (total_count, char_counts)
}

fn modulo_inverse(value: i64) -> i64 {
    let (lcoef, _) = gcd_coefs(value, MODULO);

    let inverse = lcoef % MODULO;
    if inverse < 0 {
        inverse + MODULO
    } else {
        inverse
    }
}

fn gcd_coefs(lhs: i64, rhs: i64) -> (i64, i64) {
    if lhs == 0 {
        return (0, 1);
    } else if rhs == 0 {
        return (1, 0);
    }

    match lhs.cmp(&rhs) {
        Ordering::Less => {
            let (quot, rem) = divrem(rhs, lhs);

            let (rcoef, lcoef) = gcd_coefs(rem, lhs);
            (lcoef - quot * rcoef, rcoef)
        }
        Ordering::Equal => (1, 0),
        Ordering::Greater => {
            let (rcoef, lcoef) = gcd_coefs(rhs, lhs);
            (lcoef, rcoef)
        }
    }
}

fn divrem(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2514_test() {
        check("too hot", 18);
        check("aa", 1);
        check("a", 1);
        check("abc", 6);
        check("abbbcc", 60);
    }

    fn check(input: &str, expect: i32) {
        assert_eq!(Solution::count_anagrams(input.to_string()), expect);
    }
}
