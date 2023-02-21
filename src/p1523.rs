use crate::Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let n_nums = high - low + 1;

        if n_nums % 2 == 1 {
            if low % 2 == 1 {
                (n_nums + 1) / 2
            } else {
                n_nums / 2
            }
        } else {
            n_nums / 2
        }
    }
}
