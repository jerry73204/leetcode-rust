use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut degrees = vec![0; n as usize];
        for edge in &edges {
            let (lhs, rhs) = match edge.as_slice() {
                &[lhs, rhs] => (lhs, rhs),
                _ => unreachable!(),
            };

            degrees[lhs as usize - 1] += 1;
            degrees[rhs as usize - 1] += 1;
        }

        let odd_nodes: Vec<_> = degrees
            .into_iter()
            .zip(1i32..)
            .filter(|(deg, _)| (deg & 1) == 1)
            .map(|(_, index)| index)
            .collect();

        match *odd_nodes.as_slice() {
            [] => true,
            [n1, n2] => {
                // assert n1 < n2

                // Check if edge (n1, n2) exists
                let edge_exists = edges
                    .iter()
                    .map(|edge| {
                        let (lidx, ridx) = match edge.as_slice() {
                            &[lhs, rhs] => (lhs, rhs),
                            _ => unreachable!(),
                        };

                        if lidx < ridx {
                            (lidx, ridx)
                        } else {
                            (ridx, lidx)
                        }
                    })
                    .any(|edge| edge == (n1, n2));

                if !edge_exists {
                    return true;
                }

                // Edge (n1, n2) alreadty exists. Find an even-degreed
                // node n3 and connects n1 and n2 to n3.

                let excluded: HashSet<i32> = edges
                    .iter()
                    .filter_map(|edge| {
                        if edge[0] == n1 {
                            let n3 = edge[1];
                            (n3 != n2).then(|| n3)
                        } else if edge[1] == n1 {
                            let n3 = edge[0];
                            (n3 != n2).then(|| n3)
                        } else if edge[0] == n2 {
                            let n3 = edge[1];
                            Some(n3)
                        } else if edge[1] == n2 {
                            let n3 = edge[0];
                            Some(n3)
                        } else {
                            None
                        }
                    })
                    .collect();

                excluded.len() + 2 != n as usize
            }
            [n1, n2, n3, n4] => {
                // Assert n1 < n2 < n3 < n4

                let edges: HashSet<(i32, i32)> = edges
                    .iter()
                    .map(|edge| {
                        let (lhs, rhs) = match edge.as_slice() {
                            &[lhs, rhs] => (lhs, rhs),
                            _ => unreachable!(),
                        };

                        if lhs < rhs {
                            (lhs, rhs)
                        } else {
                            (rhs, lhs)
                        }
                    })
                    .collect();

                let test_case = |e1, e2| !edges.contains(&e1) && !edges.contains(&e2);

                test_case((n1, n2), (n3, n4))
                    || test_case((n1, n3), (n2, n4))
                    || test_case((n1, n4), (n2, n3))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2508_test() {
        check(
            5,
            vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 2],
                vec![1, 4],
                vec![2, 5],
            ],
            true,
        );
        check(4, vec![vec![1, 2], vec![3, 4]], true);
        check(4, vec![vec![1, 2], vec![1, 3], vec![1, 4]], false);
        check(3, vec![vec![1, 2]], true);
        check(2, vec![vec![1, 2]], false);
    }

    fn check(n: i32, edges: Vec<Vec<i32>>, expect: bool) {
        assert_eq!(Solution::is_possible(n, edges), expect);
    }
}
