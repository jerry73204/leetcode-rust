use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as u64 - 1;
        let n = n as u64 - 1;
        let (m, n) = if m < n { (n, m) } else { (m, n) };

        let numers = ((m + 1)..=(m + n)).rev();
        let denos = 1..=n;

        numers
            .zip(denos)
            .fold(1u64, |prod, (numer, deno)| prod * numer / deno) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p62_test() {
        check(3, 2, 3);
        check(51, 9, 1916797311);
    }

    fn check(m: i32, n: i32, expect: i32) {
        assert_eq!(Solution::unique_paths(m, n), expect);
    }
}
