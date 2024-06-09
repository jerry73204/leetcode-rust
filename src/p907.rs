use crate::Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: u32 = 1_000_000_007;

        let mut seq = Vec::with_capacity(arr.len());
        let mut sum: u32 = 0;

        for (nix, &nval) in arr.iter().enumerate() {
            loop {
                let pix1 = match seq.pop() {
                    Some(last) => last,
                    None => {
                        seq.push(nix);
                        break;
                    }
                };

                let pval1 = arr[pix1];
                if pval1 < nval {
                    seq.push(pix1);
                    seq.push(nix);
                    break;
                }

                {
                    let n_lefts = match seq.last() {
                        Some(&pix2) => pix1 - pix2,
                        None => pix1 + 1,
                    };
                    let n_rights = nix - pix1;
                    let n_ranges = n_lefts * n_rights;
                    let sum_of_repeats = pval1 as u32 * n_ranges as u32;
                    sum = (sum + sum_of_repeats) % MOD;
                }
            }
        }

        while let Some(pix1) = seq.pop() {
            let n_rights = arr.len() - pix1;
            let n_lefts = match seq.last() {
                Some(&pix2) => pix1 - pix2,
                None => pix1 + 1,
            };
            let n_ranges = n_lefts * n_rights;
            let sum_of_repeats = arr[pix1] as u32 * n_ranges as u32;
            sum = (sum + sum_of_repeats) % MOD;
        }

        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p907_test() {
        check(vec![3, 1, 2, 4], 17);
        check(vec![11, 81, 94, 43, 3], 444);
        check(vec![1, 1], 3);
        check(vec![1, 2, 1, 2], 12);
        check(vec![3, 2, 1], 10);
        check(vec![11], 11);
        check(vec![51, 94, 11, 56], 318);
        check(vec![71, 55, 82, 55], 593);
    }

    fn check(arr: Vec<i32>, expect: i32) {
        assert_eq!(Solution::sum_subarray_mins(arr), expect);
    }
}
