use crate::Solution;

impl Solution {
    pub fn min_time(_n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        // Create adjacent list
        let mut nodes: Vec<_> = has_apple
            .iter()
            .map(|&has_apple| Node {
                neighbors: vec![],
                has_apple,
            })
            .collect();

        edges.iter().for_each(|edge| {
            let lhs = edge[0] as usize;
            let rhs = edge[1] as usize;
            nodes[lhs].neighbors.push(rhs);
            nodes[rhs].neighbors.push(lhs);
        });

        // Run DFS
        let DfsResult { steps, .. } = dfs(&nodes, None, 0).unwrap();

        steps
    }
}

struct Node {
    pub has_apple: bool,
    pub neighbors: Vec<usize>,
}

struct DfsResult {
    pub has_apple: bool,
    pub steps: i32,
}

fn dfs(nodes: &[Node], parent: Option<usize>, id: usize) -> Option<DfsResult> {
    // Visit children nodes and aggregate results
    let node = &nodes[id];
    let init_result = DfsResult {
        has_apple: node.has_apple,
        steps: 0,
    };
    let final_result = node
        .neighbors
        .iter()
        .copied()
        .filter(|&neighbor_id| parent != Some(neighbor_id))
        .filter_map(|neighbor_id| dfs(nodes, Some(id), neighbor_id))
        .fold(init_result, |mut acc, curr| {
            acc.has_apple |= curr.has_apple;
            if curr.has_apple {
                acc.steps += curr.steps + 2;
            }
            acc
        });

    Some(final_result)
}
