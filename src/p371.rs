use crate::Solution;

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        loop {
            let c = (a & b) << 1;
            let x = a ^ b;

            if c == 0 {
                break x;
            }

            a = x;
            b = c;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use rand::prelude::*;

    #[test]
    fn p371_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            let a = rng.gen_range(-1000..=1000);
            let b = rng.gen_range(-1000..=1000);
            let sum = a + b;
            let ans = Solution::get_sum(a, b);
            assert_eq!(ans, a + b, "{a} + {b} = {sum}, but get {ans}");
        }
    }
}
