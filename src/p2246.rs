use crate::Solution;
use std::cmp;

impl Solution {
    pub fn longest_path(parent: Vec<i32>, text: String) -> i32 {
        let nodes = {
            let mut nodes = vec![Node::default(); parent.len()];

            parent.iter().zip(text.chars()).enumerate().for_each(
                |(node_idx, (&parent_idx, symbol))| {
                    let parent_idx = if parent_idx == -1 {
                        None
                    } else {
                        Some(parent_idx as usize)
                    };
                    nodes[node_idx].parent = parent_idx;
                    nodes[node_idx].symbol = symbol;

                    if let Some(parent_idx) = parent_idx {
                        nodes[parent_idx].children.push(node_idx);
                    }
                },
            );

            nodes
        };

        // dbg!(&nodes);

        let Guess {
            longest_path_len, ..
        } = solve_recursive(&nodes, 0);

        (longest_path_len + 1) as i32
    }
}

#[derive(Debug, Clone, Default)]
struct Node {
    parent: Option<usize>,
    children: Vec<usize>,
    symbol: char,
}

#[derive(Debug)]
struct Guess {
    longest_path_len: usize,
    longest_trail_len: usize,
}

fn solve_recursive(nodes: &[Node], node_idx: usize) -> Guess {
    let node = &nodes[node_idx];

    // eprintln!("XXX {node_idx}");
    let (path_lens, trail_lens): (Vec<_>, Vec<_>) = node
        .children
        .iter()
        .copied()
        .map(|child_idx| {
            let child = &nodes[child_idx];
            let Guess {
                longest_path_len,
                longest_trail_len,
            } = solve_recursive(nodes, child_idx);

            if node.symbol != child.symbol {
                (longest_path_len, longest_trail_len + 1)
            } else {
                (longest_path_len, 0)
            }
        })
        .unzip();

    // eprintln!("YYY {node_idx} {path_lens:?} {trail_lens:?}");

    let (max_trail_len_1, max_trail_len_2) = top2(&trail_lens);
    let longest_trail_len = max_trail_len_1.unwrap_or(0);
    // dbg!(&trail_lens, (max_trail_len_1, max_trail_len_2));

    let curr_path_len = match (max_trail_len_1, max_trail_len_2) {
        (None, None) => 0,
        (Some(max), None) => max,
        (Some(max1), Some(max2)) => max1 + max2,
        (None, Some(_)) => unreachable!(),
    };

    let max_path_len_from_children = path_lens.iter().copied().max();

    let longest_path_len = match max_path_len_from_children {
        Some(max_from_children) => cmp::max(max_from_children, curr_path_len),
        None => curr_path_len,
    };

    // eprintln!("YYY {node_idx} {path_lens:?} {trail_lens:?} {longest_path_len} {longest_trail_len}");
    Guess {
        longest_path_len,
        longest_trail_len,
    }
}

fn top2(values: &[usize]) -> (Option<usize>, Option<usize>) {
    let mut iter = values.iter().copied();

    let (mut max1, mut max2) = {
        let val1 = match iter.next() {
            Some(val) => val,
            None => return (None, None),
        };

        let val2 = match iter.next() {
            Some(val) => val,
            None => return (Some(val1), None),
        };

        if val1 > val2 {
            (val1, val2)
        } else {
            (val2, val1)
        }
    };

    for val in iter {
        if val >= max1 {
            max2 = max1;
            max1 = val;
        } else if val >= max2 {
            max2 = val;
        }
    }

    (Some(max1), Some(max2))
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2246_test() {
        check(vec![-1, 0, 0, 1, 1, 2], "abacbe", 3);
        check(vec![-1, 0, 0, 0], "aabc", 3);
    }

    fn check(parent: Vec<i32>, text: &str, expect: i32) {
        assert_eq!(Solution::longest_path(parent, text.to_string()), expect);
    }
}
