use crate::Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let s = s.as_bytes();

        // Covnert the binary into run lengths of 1s and 0s.
        let mut runs = {
            let mut runs = Vec::with_capacity(s.len());
            let mut count = 1;

            for (&prev, &curr) in s.iter().zip(&s[1..]) {
                if prev == curr {
                    count += 1;
                } else {
                    runs.push(count);
                    count = 1;
                }
            }

            runs.push(count);
            runs
        };

        // Special cases
        match runs.as_slice() {
            [1] => {
                return 0;
            }
            &[n1] => {
                return n1 + 1;
            }
            &[1, n0] => {
                return n0;
            }
            &[n1, n0] => {
                return n0 + n1 + 1;
            }
            _ => {}
        }

        // Count trailing zeros
        let mut steps = if runs.len() % 2 == 0 {
            runs.pop().unwrap() + 1
        } else {
            1
        };

        loop {
            // Count 1s.
            {
                let n1 = runs.pop().unwrap();
                steps += n1;
            }

            // Count 0s.
            match runs.pop() {
                Some(n0) => {
                    steps += n0 * 2;
                }
                None => break,
            }
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1404_test() {
        check("1101", 6);
        check("10", 1);
        check("1", 0);
        check("1110", 5);
        check("11", 3);
        check("101", 5);
        check("1011000", 9);
    }

    fn check(s: &str, expect: i32) {
        assert_eq!(Solution::num_steps(s.to_string()), expect);
    }
}
