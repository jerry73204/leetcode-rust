use crate::Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let word = word.as_bytes();

        let prefix_sum_mod2 = {
            let init = 0;

            let head = [init];
            let tail = word.iter().scan(init, |oddities, &ch| {
                let nth = (ch - b'a') as usize;
                *oddities ^= 1 << nth;
                Some(*oddities)
            });

            IntoIterator::into_iter(head).chain(tail)
        };

        prefix_sum_mod2
            .scan([0i64; 1024], |counts, end_sum| {
                let all_even_count = counts[end_sum as usize];
                let one_odd_count: i64 = std::iter::successors(Some(1), |prev| Some(prev << 1))
                    .take(10)
                    .map(|mask| {
                        let start_sum = end_sum ^ mask;
                        counts[start_sum as usize]
                    })
                    .sum();
                counts[end_sum as usize] += 1;
                Some(all_even_count + one_odd_count)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1915_test() {
        check("aba", 4);
        check("aabb", 9);
        check("he", 2);
        check("a", 1);
        check("aa", 3);
        check("aaa", 6);
        check("abc", 3);
        check("abba", 8);
    }

    fn check(word: &str, expect: i64) {
        assert_eq!(Solution::wonderful_substrings(word.to_string()), expect);
    }
}
