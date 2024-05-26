use crate::Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let code_limits = {
            let mut counts = [0; 52];
            for letter in letters {
                let code = letter_to_code(letter);
                counts[code] += 1;
            }
            counts
        };

        let word_sets: Vec<_> = words
            .iter()
            .map(|word| {
                let mut char_counts = [0; 52];

                for ch in word.chars() {
                    let code = letter_to_code(ch);
                    char_counts[code] += 1;
                }

                let char_set: Vec<_> = IntoIterator::into_iter(char_counts)
                    .enumerate()
                    .filter(|(_, count)| *count > 0)
                    .collect();

                char_set
            })
            .collect();

        let mut prev_gray = 0;
        let mut code_counts = [0; 52];
        let mut total_score = 0;
        let mut invalid_count = 0;
        let mut max_score = 0;

        for bin in 1..2usize.pow(words.len() as u32) {
            let gray = binary_to_gray(bin);
            let bit = gray ^ prev_gray;
            let is_insertion = (gray & bit) != 0;

            let nth = bit.trailing_zeros() as usize;
            let code_set = &word_sets[nth];

            if is_insertion {
                for &(code, inc) in code_set {
                    let cnt = &mut code_counts[code];
                    let orig_cnt = *cnt;
                    *cnt += inc;
                    total_score += score[code] * inc;

                    let limit = code_limits[code];
                    if orig_cnt <= limit && *cnt > limit {
                        invalid_count += 1;
                    }
                }
            } else {
                for &(code, dec) in code_set {
                    let cnt = &mut code_counts[code];
                    let orig_cnt = *cnt;
                    *cnt -= dec;
                    total_score -= score[code] * dec;

                    let limit = code_limits[code];
                    if orig_cnt > limit && *cnt <= limit {
                        invalid_count -= 1;
                    }
                }
            }

            if invalid_count == 0 {
                max_score = max_score.max(total_score);
            }

            prev_gray = gray;
        }

        max_score
    }
}

fn letter_to_code(letter: char) -> usize {
    let code = letter as u8 - b'a';
    code as usize
}

fn binary_to_gray(bin: usize) -> usize {
    bin ^ (bin >> 1)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1255_test() {
        check(
            &["dog", "cat", "dad", "good"],
            &['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            &[
                1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            23,
        );
        check(
            &["xxxz", "ax", "bx", "cx"],
            &['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            &[
                4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
            ],
            27,
        );
        check(
            &["leetcode"],
            &['l', 'e', 't', 'c', 'o', 'd'],
            &[
                0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            ],
            0,
        );
    }

    fn check(words: &[&str], letters: &[char], score: &[i32], expect: i32) {
        let words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        let letters = letters.to_vec();
        let score = score.to_vec();
        assert_eq!(Solution::max_score_words(words, letters, score), expect);
    }
}
