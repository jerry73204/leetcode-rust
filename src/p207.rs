use crate::Solution;
use std::cmp;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Build the adjacent table
        let adj = {
            let mut adj = vec![vec![]; num_courses as usize];
            prerequisites.into_iter().for_each(|edge| {
                let src = edge[0];
                let dst = edge[1];
                adj[src as usize].push(dst as usize);
            });
            adj
        };

        let mut nodes = vec![Node::default(); num_courses as usize];

        for idx in 0..(num_courses as usize) {
            if nodes[idx].visited {
                continue;
            }

            let result = recursion(idx, &adj, &mut nodes);
            if result.is_err() {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Default, Clone)]
struct Node {
    pub visited: bool,
    pub level: Option<usize>,
}

fn recursion(idx: usize, adj: &[Vec<usize>], nodes: &mut [Node]) -> Result<usize, usize> {
    let node = &mut nodes[idx];

    match (node.visited, node.level) {
        (true, Some(lev)) => return Ok(lev),
        (true, None) => return Err(idx),
        (false, _) => {}
    }

    node.visited = true;

    let level = adj[idx]
        .iter()
        .map(|&nb| recursion(nb, adj, nodes))
        .try_fold(0, |max, level| -> Result<usize, usize> {
            Ok(cmp::max(max, level? + 1))
        })?;
    nodes[idx].level = Some(level);
    Ok(level)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p207_test() {
        check(
            7,
            vec![
                vec![1, 0],
                vec![0, 3],
                vec![0, 2],
                vec![3, 2],
                vec![2, 5],
                vec![4, 5],
                vec![5, 6],
                vec![2, 4],
            ],
            true,
        );
    }

    fn check(num_courses: i32, prerequisites: Vec<Vec<i32>>, expect: bool) {
        assert_eq!(Solution::can_finish(num_courses, prerequisites), expect);
    }
}
