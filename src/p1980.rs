//! # 1980. Find Unique Binary String
//! [Submission](https://leetcode.com/problems/find-unique-binary-string/submissions/1549650368)

use crate::Solution;
use std::iter::repeat;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let len = nums.len();
        let mut bitcount_freq = [0; 17];

        for num in nums {
            let n_bits = num
                .as_bytes()
                .iter()
                .filter(|&&digit| digit == b'1')
                .count();
            bitcount_freq[n_bits] += 1;
        }

        let (n_bits, _) = bitcount_freq[0..=len]
            .iter()
            .enumerate()
            .find(|&(_, &freq)| freq == 0)
            .unwrap();

        repeat('1')
            .take(n_bits)
            .chain(repeat('0'))
            .take(len)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1980_test() {
        check(&["10", "01"]);
        check(&["00", "01"]);
        check(&["111", "011", "001"]);
    }

    fn check(nums: &[&str]) {
        let nums_vec: Vec<_> = nums.iter().map(|s| s.to_string()).collect();
        let answer = Solution::find_different_binary_string(nums_vec);
        assert!(nums.iter().all(|s| s != &answer));
    }
}
