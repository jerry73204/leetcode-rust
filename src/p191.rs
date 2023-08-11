use crate::Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        // (0..u32::BITS)
        //     .filter(|bits| {
        //         let mask = 1 << bits;
        //         (n & mask) != 0
        //     })
        //     .count() as i32

        // Cheating
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use rand::prelude::*;

    #[test]
    fn p191_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            let n: u32 = rng.gen();
            let ans = Solution::hammingWeight(n);
            let expect = n.count_ones() as i32;
            assert_eq!(ans, expect);
        }
    }
}
