use crate::Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut xors = Vec::with_capacity(2usize.pow(len as u32));

        xors.push(0);

        for num in nums {
            let count = xors.len();

            for idx in 0..count {
                let xor = xors[idx] ^ num;
                xors.push(xor);
            }
        }

        xors.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3068_test() {
        check(vec![1, 3], 6);
        check(vec![5, 1, 6], 28);
        check(vec![3, 4, 5, 6, 7, 8], 480);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::subset_xor_sum(nums), expect);
    }
}
