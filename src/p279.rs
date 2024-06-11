use crate::Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut tab = vec![0; n + 1];
        tab[1] = 1;

        for val in 2..=n {
            let upper = (val as f32).sqrt().floor() as usize;

            tab[val] = (1..=upper)
                .map(|x| {
                    let remain = val - x.pow(2);
                    tab[remain]
                })
                .min()
                .unwrap()
                + 1;
        }

        tab[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p279_test() {
        check(192, 3);
    }

    fn check(n: i32, expect: i32) {
        assert_eq!(Solution::num_squares(n), expect);
    }
}
