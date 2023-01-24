use crate::Solution;

impl Solution {
    pub fn cycle_length_queries(_n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|query| {
                // Extract query
                let (lidx, ridx) = match query.as_slice() {
                    &[lidx, ridx] => (lidx as u32, ridx as u32),
                    _ => unreachable!(),
                };

                // Ensure lidx <= ridx
                let (lidx, ridx) = if lidx <= ridx {
                    (lidx, ridx)
                } else {
                    (ridx, lidx)
                };

                // Get levels of lhs and rhs nodes
                let llevel = u32::BITS - lidx.leading_zeros() - 1;
                let rlevel = u32::BITS - ridx.leading_zeros() - 1;
                debug_assert!(llevel <= rlevel);

                // Get ancestor level
                let alevel = {
                    let ridx_shifted = ridx >> (rlevel - llevel);
                    let n_leading_zeros =
                        ((lidx ^ ridx_shifted) << lidx.leading_zeros()).leading_zeros();

                    if n_leading_zeros == u32::BITS {
                        llevel
                    } else {
                        n_leading_zeros - 1
                    }
                };
                debug_assert!(alevel <= llevel);
                debug_assert!(alevel <= rlevel);

                // Compute cycle length
                let cycle_len = (llevel - alevel) + (rlevel - alevel) + 1;
                cycle_len as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2059_test() {
        check(3, vec![vec![5, 3], vec![4, 7], vec![2, 3]], vec![4, 5, 3]);
        check(2, vec![vec![1, 2]], vec![2]);
    }

    fn check(n: i32, queries: Vec<Vec<i32>>, expect: Vec<i32>) {
        assert_eq!(Solution::cycle_length_queries(n, queries), expect);
    }
}
