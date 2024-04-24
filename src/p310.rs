use crate::Solution;
use std::{cmp::Ordering::*, collections::HashSet};

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let (root, adj) = {
            let mut next = vec![None; n];

            for edge in edges.into_iter() {
                match edge.as_slice() {
                    &[ei, ej] => {
                        let ei = ei as usize;
                        let ej = ej as usize;

                        let next_i = &mut next[ei];

                        if next_i.is_none() {
                            *next_i = Some(ej);
                        } else {
                            next[ej] = Some(ei);
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let (root, _) = next
                .iter()
                .enumerate()
                .find(|(_, next)| next.is_none())
                .unwrap();

            let mut adj = vec![vec![]; n];
            next.iter()
                .enumerate()
                .filter_map(|(curr, &next)| Some((curr, next?)))
                .for_each(|(curr, next)| {
                    adj[next].push(curr);
                });

            (root, adj)
        };

        let state = {
            let mut states = State {
                max_path_len: i32::MIN,
                nodes_on_max_path: vec![],
                nodes: vec![NodeState::default(); n],
            };
            dfs1(root, &adj, &mut states);
            states
        };

        let State {
            max_path_len,
            nodes_on_max_path: start_nodes,
            nodes: node_states,
        } = state;

        if max_path_len == 0 {
            return vec![0];
        }
        let min_height = (max_path_len + 1) / 2;
        let mut answer = vec![];
        let mut visited = HashSet::new();

        for start_ix in start_nodes {
            let mut frontiers = vec![start_ix];

            while let Some(curr_ix) = frontiers.pop() {
                if !visited.insert(curr_ix) {
                    continue;
                }

                let node_state = &node_states[curr_ix];
                let curr_h = node_state.height;

                if min_height == curr_h {
                    answer.push(curr_ix as i32);

                    if max_path_len % 2 == 1 {
                        adj[curr_ix]
                            .iter()
                            .copied()
                            .filter(|&child_ix| node_states[child_ix].height + 1 == curr_h)
                            .for_each(|child_ix| {
                                answer.push(child_ix as i32);
                            });
                    }
                } else {
                    // debug_assert!(curr_h > min_height);
                    adj[curr_ix]
                        .iter()
                        .copied()
                        .filter(|&child_ix| node_states[child_ix].height + 1 == curr_h)
                        .for_each(|child_ix| {
                            frontiers.push(child_ix);
                        });
                }
            }
        }

        answer
    }
}

#[derive(Debug, Clone)]
struct State {
    pub max_path_len: i32,
    pub nodes_on_max_path: Vec<usize>,
    pub nodes: Vec<NodeState>,
}

#[derive(Debug, Clone)]
struct NodeState {
    pub height: i32,
}

impl Default for NodeState {
    fn default() -> Self {
        Self { height: i32::MIN }
    }
}

fn dfs1(curr_ix: usize, adj: &[Vec<usize>], states: &mut State) -> i32 {
    let mut top2_children = [None; 2];

    adj[curr_ix].iter().for_each(|&child_ix| {
        let child_h = dfs1(child_ix, adj, states) + 1;

        match &mut top2_children {
            [Some(first_h), Some(second_h)] => {
                if child_h > *first_h {
                    *second_h = *first_h;
                    *first_h = child_h;
                } else if child_h > *second_h {
                    *second_h = child_h;
                }
            }
            [Some(first_h), second_h @ None] => {
                if child_h > *first_h {
                    *second_h = Some(*first_h);
                    *first_h = child_h;
                } else {
                    *second_h = Some(child_h);
                }
            }
            [first_h @ None, None] => {
                *first_h = Some(child_h);
            }
            _ => unreachable!(),
        }
    });

    let (curr_height, curr_path_len) = match top2_children {
        [Some(first_h), Some(second_h)] => (first_h, first_h + second_h),
        [Some(first_h), None] => (first_h, first_h),
        [None, None] => (0, 0),
        _ => unreachable!(),
    };

    states.nodes[curr_ix] = NodeState {
        height: curr_height,
    };

    match curr_path_len.cmp(&states.max_path_len) {
        Less => {}
        Equal => {
            states.nodes_on_max_path.push(curr_ix);
        }
        Greater => {
            states.max_path_len = curr_path_len;
            states.nodes_on_max_path.clear();
            states.nodes_on_max_path.push(curr_ix);
        }
    }

    curr_height
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p310_test() {
        check(4, &[[1, 0], [1, 2], [1, 3]], &[1]);
        check(6, &[[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]], &[3, 4]);
        check(1, &[], &[0]);
        check(2, &[[0, 1]], &[0, 1]);
        check(3, &[[0, 1], [1, 2]], &[1]);
        check(4, &[[0, 1], [1, 2], [2, 3]], &[1, 2]);
    }

    fn check(n: i32, edges: &[[i32; 2]], expect: &[i32]) {
        let edges: Vec<_> = edges.iter().map(|e| e.to_vec()).collect();
        let mut answer = Solution::find_min_height_trees(n, edges);
        answer.sort_unstable();
        assert_eq!(answer, expect);
    }
}
