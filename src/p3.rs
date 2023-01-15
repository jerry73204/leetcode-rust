use crate::Solution;
use std::{cmp, collections::HashMap};

impl Solution {
    pub fn length_of_longest_substring(input: String) -> i32 {
        let mut iter = input.char_indices();
        let mut prev_state = {
            let (pos, ch) = match iter.next() {
                Some(item) => item,
                None => return 0,
            };
            State::with_char(ch, pos)
        };
        let mut max_len = prev_state.len;

        // dbg!(&prev_state);

        for (curr_pos, ch) in iter {
            let mut curr_state = prev_state;
            let pos_opt = curr_state.position_of_char(ch);

            match pos_opt {
                Some(prev_pos_ref) => {
                    let prev_pos = *prev_pos_ref;
                    let new_len = curr_pos - prev_pos;
                    *prev_pos_ref = curr_pos;
                    curr_state.len = new_len;
                    curr_state.remove_char_before_position(prev_pos);
                }
                None => {
                    // *pos_opt = Some(curr_pos);
                    curr_state.set_char_pos(ch, curr_pos);
                    curr_state.len += 1;
                }
            }

            max_len = cmp::max(max_len, curr_state.len);
            prev_state = curr_state;
            // dbg!(&prev_state);
        }

        max_len as i32
    }
}

#[derive(Debug, Clone)]
struct State {
    len: usize,
    char_positions: HashMap<char, usize>,
}

impl State {
    pub fn with_char(ch: char, position: usize) -> Self {
        // let index = char_to_index(ch);
        let mut char_positions = HashMap::new();
        char_positions.insert(ch, position);
        Self {
            len: 1,
            char_positions,
        }
    }

    pub fn set_char_pos(&mut self, ch: char, pos: usize) {
        self.char_positions.insert(ch, pos);
    }

    pub fn position_of_char(&mut self, ch: char) -> Option<&mut usize> {
        // let index = char_to_index(ch);
        self.char_positions.get_mut(&ch)
    }

    pub fn remove_char_before_position(&mut self, last_pos: usize) {
        self.char_positions.retain(|_ch, pos| *pos >= last_pos);

        // self.char_positions.iter_mut().for_each(|pos_opt| {
        //     if let Some(pos) = *pos_opt {
        //         if pos < last_pos {
        //             *pos_opt = None;
        //         }
        //     }
        // });
    }
}

// fn char_to_index(ch: char) -> usize {
//     // // a-z: 0-25
//     // // A-Z: 26-51
//     // // 0-9: 52-61
//     // // ' ': 62

//     // let index = if ch.is_ascii_lowercase() {
//     //     ch as u32 - 'a' as u32
//     // } else if ch.is_ascii_uppercase() {
//     //     ch as u32 - 'A' as u32 + 26
//     // } else if ch.is_ascii_digit() {
//     //     ch as u32 - '0' as u32 + 52
//     // } else {
//     //     62
//     // };
//     // index as usize
//     ch as u32 as usize
// }

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3_test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
        assert_eq!(
            Solution::length_of_longest_substring("skqsjdxicylkltdamomobovngougjxpb".to_string()),
            10
        );
    }
}
