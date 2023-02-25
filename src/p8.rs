use crate::Solution;

impl Solution {
    pub fn my_atoi(input: String) -> i32 {
        let bytes = input.as_bytes();

        let mut iter = bytes
            .iter()
            .copied()
            .skip_while(|&ch| ch == b' ')
            .peekable();

        let ch = iter.next_if(|ch| [b'-', b'+'].contains(ch));
        let is_negative = ch == Some(b'-');

        let mut curr: u64 = 0;

        while let Some(ch) = iter.next_if(|ch| ch.is_ascii_digit()) {
            let digit = ch - b'0';
            curr = curr * 10 + digit as u64;

            if is_negative {
                if curr > (1 << 31) {
                    return i32::MIN;
                }
            } else if curr > ((1 << 31) - 1) {
                return i32::MAX;
            }
        }

        if is_negative {
            (-(curr as i64)) as i32
        } else {
            curr as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p8_test() {
        check("42", 42);
        check("-42", -42);
        check("4193 with words", 4193);
        check("", 0);
        check("0032", 32);
        check("0-32", 0);
        check("+1", 1);
        check("-1", -1);
        check("2147483647", 2147483647);
        check("2147483648", 2147483647);
        check("21474836470", 2147483647);
        check("-2147483648", -2147483648);
        check("-2147483649", -2147483648);
        check("-21474836480", -2147483648);
    }

    fn check(input: &str, expect: i32) {
        assert_eq!(Solution::my_atoi(input.to_string()), expect);
    }
}
