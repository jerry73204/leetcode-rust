use crate::Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let x = k % 10;
        let x_inv = match x {
            0 | 2 | 4 | 5 | 6 | 8 => return -1,
            1 => 1,
            3 => 7,
            7 => 3,
            9 => 9,
            _ => unreachable!(),
        };

        let mut carry = (x_inv * k) / 10;
        let mut cnt = 1;

        while carry > 0 {
            let y = (x_inv * (1 - carry)) % 10;
            let y = if y < 0 { y + 10 } else { y };
            carry = (carry + y * k) / 10;
            cnt += 1;
        }

        cnt
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1015_test() {
        check(1, 1);
        check(2, -1);
        check(3, 3);
        check(4, -1);
        check(21, 6);
    }

    fn check(k: i32, expect: i32) {
        assert_eq!(Solution::smallest_repunit_div_by_k(k), expect);
    }
}
