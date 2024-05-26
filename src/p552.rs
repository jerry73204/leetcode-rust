use crate::Solution;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        let p = 10u64.pow(9) + 7;

        // A table indexed by (nth, attendance code)
        // The attendance code represents the attendance of last few days.
        // 0 => ...A
        // 1 => ..XL, X = A or start of string
        // 2 => .XLL, X = A or start of string
        // 3 => ...., any of above
        let mut tab = vec![[0u64; 4]; n + 1];

        tab[0] = [1, 0, 0, 1];

        for idx in 1..=n {
            let prev: [_; 4] = tab[idx - 1];
            let next: &mut [_; 4] = &mut tab[idx];
            let [pa, pl, pll, _] = prev;
            let [na, nl, nll, nall] = next;

            *na = (pa + pl + pll) % p;
            *nl = pa;
            *nll = pl;
            *nall = (*na + *nl + *nll) % p;
        }

        // Count the cases where there is no 'A'.
        let count_without_a: u64 = tab[n][3];

        // Count cases where one 'A' is at nth day.
        let count_with_a = {
            let iter1 = tab.iter().map(|t| t[3]);
            let iter2 = tab[0..n].iter().rev().map(|t| t[3]);

            let sum = iter1
                .zip(iter2)
                .take(n / 2)
                .map(|(lcnt, rcnt)| (lcnt * rcnt) % p)
                .reduce(|l, r| (l + r) % p)
                .unwrap_or(0);

            let sum = (sum * 2) % p;

            if (n % 2) != 0 {
                let nth = n / 2;
                (sum + tab[nth][3].pow(2)) % p
            } else {
                sum
            }
        };

        ((count_with_a + count_without_a) % p) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p552_test() {
        check(2, 8);
        check(1, 3);
        check(10101, 183236316);
    }

    fn check(n: i32, expect: i32) {
        assert_eq!(Solution::check_record(n), expect);
    }
}
