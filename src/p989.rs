use crate::Solution;

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut carry = k;

        for digit in num.iter_mut().rev() {
            *digit += carry;
            carry = *digit / 10;
            *digit %= 10;

            if carry == 0 {
                return num;
            }
        }

        let mut prepends = vec![];

        while carry > 0 {
            prepends.push(carry % 10);
            carry /= 10;
        }

        prepends.into_iter().rev().chain(num).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p989_test() {
        check(vec![1, 2, 0, 0], 34, vec![1, 2, 3, 4]);
        check(vec![2, 7, 4], 181, vec![4, 5, 5]);
        check(vec![2, 1, 5], 806, vec![1, 0, 2, 1]);
        check(vec![0], 1000, vec![1, 0, 0, 0]);
    }

    fn check(nums: Vec<i32>, k: i32, expect: Vec<i32>) {
        assert_eq!(Solution::add_to_array_form(nums, k), expect);
    }
}
