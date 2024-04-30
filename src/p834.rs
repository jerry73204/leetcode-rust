use crate::Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        // Build adjacent table and locate the root node
        let (root, adj) = {
            let mut adj = vec![vec![]; n];

            for edge in edges {
                let [a, b] = match *edge {
                    [a, b] => [a as usize, b as usize],
                    _ => unreachable!(),
                };

                adj[a].push(b);
                adj[b].push(a);
            }

            (0, adj)
        };

        // Count the # of nodes and total distance to descending nodes per subtree
        let mut tree_sizes = vec![0; n];
        let (total_n_nodes, total_dist_at_root) = dfs1(root, None, &adj, &mut tree_sizes);

        // The (1) total # of nodes and (2) the total distance from root to others are known.
        // Run the 2nd DFS to find the answer.
        let mut total_dists = vec![0; n];
        total_dists[root] = total_dist_at_root;

        for &child in &adj[root] {
            dfs2(
                child,
                root,
                &adj,
                total_n_nodes,
                total_dist_at_root,
                &tree_sizes,
                &mut total_dists,
            );
        }

        total_dists
    }
}

/// Return the number of descending nodes and the total distance to
/// descending nodes on the current node.
fn dfs1(
    curr: usize,
    parent: Option<usize>,
    adj: &[Vec<usize>],
    tree_sizes: &mut [i32],
) -> (i32, i32) {
    let mut tree_size = 1;
    let mut tree_dist = 0;

    for &child in &adj[curr] {
        if Some(child) == parent {
            continue;
        }

        let (subtree_size, subtree_total_dist) = dfs1(child, Some(curr), adj, tree_sizes);
        tree_size += subtree_size;
        tree_dist += subtree_total_dist + subtree_size;
    }

    tree_sizes[curr] = tree_size;
    (tree_size, tree_dist)
}

fn dfs2(
    curr: usize,
    parent: usize,
    adj: &[Vec<usize>],
    total_n_nodes: i32,
    parent_total_dist: i32,
    tree_sizes: &[i32],
    total_dists: &mut [i32],
) {
    let tree_size = tree_sizes[curr];
    let parent_n_nodes = total_n_nodes - tree_size;
    let total_dist = parent_total_dist + parent_n_nodes - tree_size;
    total_dists[curr] = total_dist;

    for &child in &adj[curr] {
        if child == parent {
            continue;
        }

        dfs2(
            child,
            curr,
            adj,
            total_n_nodes,
            total_dist,
            tree_sizes,
            total_dists,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p834_test() {
        check(
            6,
            &[[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]],
            &[8, 12, 6, 10, 10, 10],
        );
        check(1, &[], &[0]);
        check(2, &[[1, 0]], &[1, 1]);
        check(5, &[[2, 0], [4, 2], [3, 1], [1, 0]], &[6, 7, 7, 10, 10]);
    }

    fn check(n: i32, edges: &[[i32; 2]], expect: &[i32]) {
        let edges: Vec<_> = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), expect);
    }
}
