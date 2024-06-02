use crate::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let a_xor_b: i32 = nums.iter().copied().reduce(|a, b| a ^ b).unwrap();
        let bit = 1i32 << a_xor_b.trailing_zeros();

        let [xor0, xor1] = nums.iter().copied().fold([0, 0], |mut xors, val| {
            let [xor0, xor1] = &mut xors;

            if val & bit == 0 {
                *xor0 ^= val;
            } else {
                *xor1 ^= val;
            }

            xors
        });

        vec![xor0, xor1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p260_test() {
        check(vec![1, 2, 1, 3, 2, 5], [3, 5]);
        check(vec![-1, 0], [-1, 0]);
        check(vec![0, 1], [0, 1]);
    }

    fn check(nums: Vec<i32>, mut expect: [i32; 2]) {
        let mut answer = Solution::single_number(nums);
        answer.sort_unstable();
        expect.sort_unstable();
        assert_eq!(answer, expect.to_vec());
    }
}
