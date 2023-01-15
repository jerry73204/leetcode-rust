use crate::Solution;
use std::{cmp::Ordering::*, collections::HashMap};

type NodeIdx = usize;
type Value = i32;
type AdjMap = HashMap<NodeIdx, Vec<NodeIdx>>;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<Value>, edges: Vec<Vec<i32>>) -> i32 {
        let mut nodes: Vec<_> = vals
            .into_iter()
            .enumerate()
            .map(|(index, value)| Node {
                // index,
                value,
                parent: index,
                rank: 0,
            })
            .collect();

        let adj: AdjMap = {
            let mut adj: AdjMap = HashMap::new();

            for edge in edges {
                let src = edge[0] as usize;
                let dst = edge[1] as usize;

                adj.entry(src)
                    .and_modify(|neighbors| {
                        neighbors.push(dst);
                    })
                    .or_insert_with(|| vec![dst]);
                adj.entry(dst)
                    .and_modify(|neighbors| {
                        neighbors.push(src);
                    })
                    .or_insert_with(|| vec![src]);
            }

            adj
        };

        let sorted_indices = {
            let mut indices: Vec<_> = (0..nodes.len()).collect();
            indices.sort_unstable_by_key(|&idx| nodes[idx].value);
            indices
        };

        let groups: Vec<Group<'_>> =
            Uniq::new(&sorted_indices, |node_idx| nodes[node_idx].value).collect();

        // Process node groups from small values to large values
        let num_paths: usize = groups
            .into_iter()
            .map(|group| {
                let Group {
                    value: cur_value,
                    indices,
                } = group;

                // Union nodes with the same value
                indices.iter().for_each(|&cur_idx| {
                    let neighbors_indices = adj.get(&cur_idx).into_iter().flatten();

                    neighbors_indices.for_each(|&neighbor_idx| {
                        let neighbor = &nodes[neighbor_idx];
                        if neighbor.value <= cur_value {
                            union(&mut nodes, cur_idx, neighbor_idx);
                            // assert!(find(&nodes, cur_idx) == find(&nodes, neighbor_idx));
                        }
                    });
                });

                // dbg!((cur_value, &nodes));

                // eprintln!("value={} count={}", cur_value, indices.len());
                // nodes.iter().for_each(|node| {
                //     eprintln!("{}\t{}\t{}", node.index, node.parent, node.value);
                // });
                // eprintln!();

                // Group nodes by their parents
                let counts: HashMap<NodeIdx, usize> = indices
                    .iter()
                    .map(|&cur_idx| find(&nodes, cur_idx))
                    .fold(HashMap::new(), |mut counts, parent| {
                        counts
                            .entry(parent)
                            .and_modify(|count| {
                                *count += 1;
                            })
                            .or_insert(1);
                        counts
                    });

                let num_paths_for_value: usize = counts
                    .into_iter()
                    .map(|(_, count)| count * (count - 1) / 2)
                    .sum();

                num_paths_for_value
            })
            .sum();

        let answer = num_paths + nodes.len();
        answer as i32
    }
}

#[derive(Debug)]
struct Node {
    // pub index: usize,
    pub value: Value,
    pub parent: NodeIdx,
    pub rank: u32,
}

struct Group<'a> {
    pub value: Value,
    pub indices: &'a [NodeIdx],
}

struct Uniq<'a, F>
where
    F: FnMut(NodeIdx) -> Value,
{
    slice: &'a [NodeIdx],
    end: usize,
    key_fn: F,
}

impl<'a, F> Uniq<'a, F>
where
    F: FnMut(NodeIdx) -> Value,
{
    pub fn new(slice: &'a [NodeIdx], key_fn: F) -> Self {
        Self {
            slice,
            end: 0,
            key_fn,
        }
    }
}

impl<'a, F> Iterator for Uniq<'a, F>
where
    F: FnMut(NodeIdx) -> Value,
{
    type Item = Group<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end == self.slice.len() {
            return None;
        }
        let value = (self.key_fn)(self.slice[self.end]);
        let start = self.end;
        let mut end = start + 1;

        while end < self.slice.len() && (self.key_fn)(self.slice[end]) == value {
            end += 1;
        }

        self.end = end;

        Some(Group {
            value,
            indices: &self.slice[start..end],
        })
    }
}

fn find(nodes: &[Node], mut node_idx: NodeIdx) -> NodeIdx {
    loop {
        let node = &nodes[node_idx];
        if node.parent == node_idx {
            return node_idx;
        }
        node_idx = node.parent;
    }
}

fn union(nodes: &mut [Node], lidx: NodeIdx, ridx: NodeIdx) {
    // eprintln!("union {} {}", lidx, ridx);
    if lidx == ridx {
        return;
    }

    let lidx = find(nodes, lidx);
    let ridx = find(nodes, ridx);

    let (lidx, ridx) = match lidx.cmp(&ridx) {
        Less => (lidx, ridx),
        Equal => return,
        Greater => (ridx, lidx),
    };

    let (lnode, rnode) = {
        let slice = &mut nodes[lidx..=ridx];
        let (lnode, slice) = slice.split_first_mut().unwrap();
        let (rnode, _) = slice.split_last_mut().unwrap();
        (lnode, rnode)
    };

    let ((lnode, rnode), (lidx, _ridx)) = if lnode.rank >= rnode.rank {
        ((lnode, rnode), (lidx, ridx))
    } else {
        ((rnode, lnode), (ridx, lidx))
    };

    rnode.parent = lidx;

    if lnode.rank == rnode.rank {
        lnode.rank += 1
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use crate::Solution;

    #[test]
    fn p2421_test() {
        check(
            vec![1, 3, 2, 1, 3],
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            6,
        );
        check(
            vec![1, 1, 2, 2, 3],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]],
            7,
        );
        check(vec![1], vec![], 1);
        check(
            vec![1, 4, 4, 5, 4, 3, 3, 5, 3, 2],
            vec![
                vec![1, 0],
                vec![2, 1],
                vec![1, 3],
                vec![3, 4],
                vec![5, 4],
                vec![6, 2],
                vec![7, 5],
                vec![8, 4],
                vec![8, 9],
            ],
            12,
        );
    }

    fn check(vals: Vec<Value>, edges: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::number_of_good_paths(vals, edges), expect);
    }
}
