use crate::Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let letter_to_code = |letter: u8| (letter - b'a') as usize;
        let code_to_letter = |code: usize| code as u8 + b'a';

        let letter_freq = words
            .into_iter()
            .map(|word| {
                let mut letter_freq = [0; 26];

                for &letter in word.as_bytes() {
                    let code = letter_to_code(letter);
                    letter_freq[code] += 1;
                }

                letter_freq
            })
            .reduce(|mut lfreq, rfreq| {
                lfreq.iter_mut().zip(rfreq).for_each(|(lcnt, rcnt)| {
                    *lcnt = (*lcnt).min(rcnt);
                });
                lfreq
            })
            .unwrap();

        letter_freq
            .iter()
            .enumerate()
            .flat_map(|(code, &repeat)| std::iter::repeat(code).take(repeat))
            .map(|code| {
                let letter = code_to_letter(code) as char;
                letter.to_string()
            })
            .collect()
    }
}
