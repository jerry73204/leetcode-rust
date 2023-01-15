use crate::Solution;

const FACTORIALS: [i32; 9] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320];

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut quotients = vec![0; n as usize];
        decompose(k - 1, n - 1, &mut quotients);

        // eprintln!("problem: {n} {k}");
        // quotients
        //     .iter()
        //     .zip(&FACTORIALS)
        //     .enumerate()
        //     .for_each(|(index, (&quot, &fac))| {
        //         eprintln!("{index}\t{quot}\t{fac}");
        //     });

        let digits: Vec<u8> = {
            let mut digits: Vec<_> = (1u8..=(n as u8)).collect();
            let mut slice = digits.as_mut_slice();

            for quot in quotients.iter().clone().rev() {
                let size = (quot + 1) as usize;
                slice[..size].rotate_right(1);
                slice = &mut slice[1..];
            }
            digits
        };

        digits
            .into_iter()
            .map(|digit| (digit + b'0') as char)
            .collect()
    }
}

fn decompose(index: i32, level: i32, quotients: &mut [i32]) {
    if level == 0 {
        debug_assert!(index == 0);
        quotients[0] = 0;
        return;
    }

    let divisor = FACTORIALS[level as usize];
    let quot = index / divisor;
    let rem = index % divisor;

    quotients[level as usize] = quot;
    decompose(rem, level - 1, quotients);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p60_test() {
        check(1, 1, "1");
        check(2, 1, "12");
        check(2, 2, "21");
        check(3, 1, "123");
        check(3, 2, "132");
        check(3, 3, "213");
        check(3, 4, "231");
        check(3, 5, "312");
        check(3, 6, "321");
        check(4, 9, "2314");
    }

    fn check(n: i32, k: i32, expect: &str) {
        assert_eq!(Solution::get_permutation(n, k), expect);
    }
}
