use crate::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let digits: Vec<u32> = s.chars().map(|c| c as u32 - '0' as u32).collect();
        let mut count = vec![0; digits.len() + 1];

        count[digits.len()] = 1;
        {
            let last_digit = *digits.last().unwrap();
            count[digits.len() - 1] = if (1..=9).contains(&last_digit) { 1 } else { 0 };
        }

        for idx in (0..(digits.len() - 1)).rev() {
            // Case: 1 digit
            let code = digits[idx];

            let case1_cnt = if (1..=9).contains(&code) {
                count[idx + 1]
            } else {
                0
            };

            // Case: 2 digits
            let code = digits[idx] * 10 + digits[idx + 1];
            let case2_cnt = if (10..=26).contains(&code) {
                count[idx + 2]
            } else {
                0
            };

            count[idx] = case1_cnt + case2_cnt;
        }

        count[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p91_test() {
        check("12", 2);
        check("226", 3);
        check("06", 0);
        check("27", 1);
    }

    fn check(s: &str, expect: i32) {
        assert_eq!(Solution::num_decodings(s.to_string()), expect);
    }
}
