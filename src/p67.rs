use crate::Solution;

impl Solution {
    pub fn add_binary(a_str: String, b_str: String) -> String {
        let a_bytes = a_str.as_bytes();
        let b_bytes = b_str.as_bytes();

        let mut a_iter = a_bytes.iter().rev().copied();
        let mut b_iter = b_bytes.iter().rev().copied();

        let mut carry = false;
        let mut sum_bits = vec![];

        loop {
            let a_digit = a_iter.next();
            let b_digit = b_iter.next();

            let (a_bit, b_bit) = match (a_digit, b_digit) {
                (None, None) => break,
                (None, Some(b_digit)) => (false, b_digit == b'1'),
                (Some(a_digit), None) => (a_digit == b'1', false),
                (Some(a_digit), Some(b_digit)) => (a_digit == b'1', b_digit == b'1'),
            };

            let (new_carry, new_bit) = adder(a_bit, b_bit, carry);
            carry = new_carry;
            sum_bits.push(new_bit);
        }

        if carry {
            sum_bits.push(true);
        }

        sum_bits
            .into_iter()
            .rev()
            .map(|bit| if bit { '1' } else { '0' })
            .collect()
    }
}

fn adder(abit: bool, bbit: bool, carry: bool) -> (bool, bool) {
    let new_carry = (carry || bbit) && abit || (bbit && carry);
    let new_bit = abit ^ bbit ^ carry;
    (new_carry, new_bit)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p67_test() {
        check("11", "1", "100");
        check("1010", "1011", "10101");
    }

    fn check(astr: &str, bstr: &str, expect: &str) {
        assert_eq!(
            Solution::add_binary(astr.to_string(), bstr.to_string()),
            expect
        );
    }
}
