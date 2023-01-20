use crate::Solution;
use std::cmp;

impl Solution {
    pub fn min_flips_mono_incr(input: String) -> i32 {
        struct State {
            acc_flip: i32,
            min_acc_flip: i32,
            acc_zeros: i32,
        }

        let init_state = State {
            acc_flip: 0,
            min_acc_flip: 0,
            acc_zeros: 0,
        };

        let final_state = input.chars().fold(init_state, |mut state, ch| {
            if ch == '0' {
                state.acc_zeros += 1;
                state.acc_flip -= 1;
            } else {
                debug_assert_eq!(ch, '1');
                state.acc_flip += 1;
            };
            state.min_acc_flip = cmp::min(state.min_acc_flip, state.acc_flip);
            state
        });

        let State {
            acc_zeros,
            min_acc_flip,
            ..
        } = final_state;

        min_acc_flip + acc_zeros
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p926_test() {
        check("00110", 1);
        check("010110", 2);
        check("00011000", 2);
        check("0", 0);
        check("1", 0);
        check("0000", 0);
        check("1111", 0);
        check("11011", 1);
        check("00100", 1);
    }

    fn check(input: &str, expect: i32) {
        assert_eq!(Solution::min_flips_mono_incr(input.to_string()), expect);
    }
}
