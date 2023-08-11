use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let nrow = heights.len();
        let ncol = heights[0].len();

        let get_neighbors = |r: usize, c: usize| {
            let curr_height = heights[r][c];
            let left = r.checked_sub(1).map(|r_sub| (r_sub, c));
            let top = c.checked_sub(1).map(|c_sub| (r, c_sub));
            let right = (r + 1 < nrow).then(|| (r + 1, c));
            let bottom = (c + 1 < ncol).then(|| (r, c + 1));

            let heights = &heights;
            left.into_iter()
                .chain(top)
                .chain(right)
                .chain(bottom)
                .filter(move |&(r, c)| curr_height <= heights[r][c])
        };

        let mut nodes = vec![vec![Node::default(); ncol]; nrow];

        // Flood from Pacific
        {
            let mut frontiers: VecDeque<(usize, usize)> = VecDeque::new();
            frontiers.push_back((0, 0));
            frontiers.extend((1..nrow).map(|r| (r, 0)));
            frontiers.extend((1..ncol).map(|c| (0, c)));

            while let Some((r, c)) = frontiers.pop_front() {
                let mut node = &mut nodes[r][c];

                if node.reach_pacific {
                    continue;
                }
                node.reach_pacific = true;
                frontiers.extend(get_neighbors(r, c));
            }
        }

        // Flood from Atlantic
        {
            let mut answer = vec![];

            let mut frontiers: VecDeque<(usize, usize)> = VecDeque::new();
            frontiers.push_back((nrow - 1, ncol - 1));
            frontiers.extend((0..(nrow - 1)).rev().map(|r| (r, ncol - 1)));
            frontiers.extend((0..(ncol - 1)).rev().map(|c| (nrow - 1, c)));

            while let Some((r, c)) = frontiers.pop_front() {
                let mut node = &mut nodes[r][c];

                if node.reach_atlantic {
                    continue;
                }
                node.reach_atlantic = true;
                frontiers.extend(get_neighbors(r, c));

                if node.reach_pacific {
                    answer.push(vec![r as i32, c as i32]);
                }
            }

            answer
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Node {
    pub reach_pacific: bool,
    pub reach_atlantic: bool,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p417_test() {
        check(
            vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4],
            ],
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0],
            ],
        );
    }

    fn check(heights: Vec<Vec<i32>>, mut expect: Vec<Vec<i32>>) {
        let mut answer = Solution::pacific_atlantic(heights);

        expect.sort_unstable();
        answer.sort_unstable();
        assert_eq!(answer, expect);
    }
}
