use crate::Solution;
use std::{cmp, convert::TryInto};

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n_nodes = edges.len();
        let mut marks = vec![
            Mark {
                dist1: None,
                dist2: None
            };
            n_nodes
        ];

        // Scan descendents from node1
        let entry1 = {
            let mut index = node1 as usize;
            let mut dist_iter = 0..;

            loop {
                let mark = &mut marks[index];

                // If the node was visited, the path has a loop.
                // Remember the loop entry node.
                if let Some(entry_dist) = mark.dist1 {
                    let exit_dist = dist_iter.next().unwrap();
                    break Some((index, entry_dist, exit_dist));
                }

                // Save the distance from node1 on the current node.
                mark.dist1 = Some(dist_iter.next().unwrap());

                // Get the suceeding node. If the suceeding node does
                // not exist, the path does not have a loop. Exit the
                // loop.
                let next: Option<usize> = edges[index].try_into().ok();
                let next = match next {
                    Some(next) => next,
                    None => break None,
                };
                index = next;
            }
        };

        // Scan descendents from node2.
        {
            let mut index = node2 as usize;
            let mut dist_iter = 0..;

            loop {
                let mark = &mut marks[index];

                // Check if the node is reachable from node1. If yes,
                // check if the path has a loop.
                if let Some(meet_dist1) = mark.dist1 {
                    if let Some((entry_index1, entry_dist1, exit_dist1)) = entry1 {
                        let meet_dist2 = dist_iter.next().unwrap();

                        let candidate1 = Candidate {
                            max_distance: cmp::max(meet_dist1, meet_dist2),
                            meet_index: index,
                        };
                        let candidate2 = Candidate {
                            max_distance: cmp::max(
                                entry_dist1,
                                exit_dist1 - meet_dist1 + meet_dist2,
                            ),
                            meet_index: entry_index1,
                        };

                        let winner = cmp::min(candidate1, candidate2);
                        return winner.meet_index as i32;
                    } else {
                        // If there is no loop, current node is the
                        // closest node.
                        return index as i32;
                    }
                }

                // If the node was visited before reaching to any
                // node1 descendents, conclude that there is no node
                // reachable from both node1 and node2.
                if mark.dist2.is_some() {
                    return -1;
                }

                mark.dist2 = Some(dist_iter.next().unwrap());

                // Get the suceeding node. If the suceeding node does
                // not exist, conclude that there is no node reachable
                // from both node1 and node2.
                let next: Option<usize> = edges[index].try_into().ok();
                let next = match next {
                    Some(next) => next,
                    None => return -1,
                };
                index = next;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Mark {
    pub dist1: Option<usize>,
    pub dist2: Option<usize>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Candidate {
    pub max_distance: usize,
    pub meet_index: usize,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2359_test() {
        check(vec![2, 2, 3, -1], 0, 1, 2);
        check(vec![1, 2, -1], 0, 2, 2);
        check(vec![-1], 0, 0, 0);
        check(vec![1, 0], 0, 1, 0);
        check(vec![4, 4, 8, -1, 9, 8, 4, 4, 1, 1], 5, 6, 1);
    }

    fn check(edges: Vec<i32>, node1: i32, node2: i32, expect: i32) {
        assert_eq!(Solution::closest_meeting_node(edges, node1, node2), expect);
    }
}
