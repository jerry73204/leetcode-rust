//! # 2467. Most Profitable Path in a Tree
//! [Submission](https://leetcode.com/problems/most-profitable-path-in-a-tree/submissions/1553911516)

use crate::Solution;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, mut amount: Vec<i32>) -> i32 {
        let n = edges.len() + 1;
        let bob = bob as usize;

        // Construct the adjacent table
        let adj = {
            let mut adj: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n);
            edges
                .into_iter()
                .flat_map(|edge| {
                    let &[a, b] = edge.as_slice() else {
                        unreachable!()
                    };
                    let a = a as usize;
                    let b = b as usize;
                    [[a, b], [b, a]]
                })
                .for_each(|[a, b]| {
                    adj.entry(a)
                        .or_insert_with(|| Vec::with_capacity(8))
                        .push(b);
                });
            adj
        };

        // Find the rendezvous node
        {
            let mut parents = vec![0; n]; // Records the parents of visited nodes.

            // Run BFS
            let mut queue = VecDeque::with_capacity(n);
            queue.extend(adj[&0].iter().map(|&next| (0, 1, next)));

            let bob_depth = loop {
                let (parent, depth, curr) = queue.pop_front().unwrap();
                parents[curr] = parent;

                if curr == bob {
                    // If reachnig the Bob node, leave the loop and
                    // record the depth.
                    break depth;
                } else {
                    // Otherwise, schedule visits to children nodes.
                    queue.extend(
                        adj[&curr]
                            .iter()
                            .filter(|&&next| next != parent)
                            .map(|&next| (curr, depth + 1, next)),
                    );
                }
            };

            // Climb from Bob node towards the root to the half-depth
            // node to find the rendezvous node.
            let steps = bob_depth / 2;
            let last = (0..steps).fold(bob, |curr, _| {
                amount[curr] = 0;
                parents[curr]
            });

            // Alice and Bob will meet if the depth of Bob node is
            // even.
            let will_meet = (bob_depth & 1) == 0;
            if will_meet {
                amount[last] /= 2;
            } else {
                amount[last] = 0;
            }
        }

        #[derive(Debug)]
        struct Job {
            parent: Option<usize>,
            curr: usize,
            depth: usize,
            income: i32,
        }

        let mut trace = Vec::with_capacity(n);
        let mut stack = Vec::with_capacity(n);
        let mut max_income = i32::MIN;

        stack.push(Job {
            parent: None,
            curr: 0,
            depth: 0,
            income: 0,
        });

        while let Some(job) = stack.pop() {
            let Job {
                parent,
                curr,
                depth,
                income: parent_income,
            } = job;

            trace.resize_with(depth, || unreachable!());
            trace.push(curr);

            let curr_income = parent_income + amount[curr];

            let is_leaf = matches!(
            (parent, adj[&curr].as_slice()),
            (Some(parent), &[next]) if parent == next,
                );

            if is_leaf {
                max_income = std::cmp::max(max_income, curr_income);
            } else {
                let child_depth = depth + 1;

                stack.extend(adj[&curr].iter().filter(|&&node| Some(node) != parent).map(
                    |&child| Job {
                        parent: Some(curr),
                        curr: child,
                        depth: child_depth,
                        income: curr_income,
                    },
                ));
            }
        }

        max_income
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2467_test() {
        check(&[[0, 1], [1, 2], [1, 3], [3, 4]], 3, &[-2, 4, 2, -4, 6], 6);
        check(&[[0, 1]], 1, &[-7280, 2350], -7280);
        check(
            &[[0, 2], [0, 6], [1, 3], [1, 5], [2, 5], [4, 6]],
            6,
            &[8838, -6396, -5940, 2694, -1366, 4616, 2966],
            7472,
        );
        check(
            &[
                [0, 2],
                [1, 8],
                [1, 6],
                [2, 3],
                [2, 4],
                [3, 7],
                [4, 5],
                [4, 9],
                [6, 7],
            ],
            3,
            &[-378, -3744, -638, 9938, 3818, -336, 2722, 154, -1750, -1672],
            2785,
        );
    }

    fn check(edges: &[[i32; 2]], bob: i32, amount: &[i32], expect: i32) {
        let edges: Vec<_> = edges.iter().map(|e| e.to_vec()).collect();
        let amount = amount.to_vec();

        assert_eq!(Solution::most_profitable_path(edges, bob, amount), expect);
    }
}
