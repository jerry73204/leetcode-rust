use crate::Solution;
use std::{collections::HashMap, iter};

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let size = board.len();
        let size_sq = size.pow(2);

        // Convert input board to index-to-destination map
        let ladders: HashMap<usize, usize> = {
            let reverse_iter = [false, true].into_iter().cycle();
            board
                .into_iter()
                .rev()
                .zip(reverse_iter)
                .flat_map(|(row, reverse)| {
                    let row_iter: Box<dyn Iterator<Item = i32>> = if reverse {
                        Box::new(row.into_iter().rev())
                    } else {
                        Box::new(row.into_iter())
                    };
                    row_iter
                })
                .enumerate()
                .filter_map(|(idx, ladder)| {
                    if ladder == -1 {
                        return None;
                    }

                    Some((idx, (ladder - 1) as usize))
                })
                .collect()
        };

        // Construct adjacent map
        let adj: HashMap<usize, Vec<usize>> = {
            let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();

            (0..size_sq)
                .flat_map(|idx| {
                    let nexts = {
                        let first = idx + 1;
                        let last = (idx + 6).min(size_sq - 1);
                        first..=last
                    };

                    let dests = nexts.map(|next_idx| match ladders.get(&next_idx) {
                        Some(&ladder) => ladder,
                        None => next_idx,
                    });

                    dests.map(move |dest_idx| (idx, dest_idx))
                })
                .for_each(|(src, dst)| {
                    adj.entry(src)
                        .and_modify(|dsts| {
                            dsts.push(dst);
                        })
                        .or_insert_with(|| vec![dst]);
                });

            adj
        };

        // dbg!(&adj);

        // Run Bellman-Ford
        let mut distances = vec![i32::MAX; size_sq];
        distances[size_sq - 1] = 0;

        for _ in 0..(size_sq - 1) {
            let mut is_updated = false;

            (0..size_sq).rev().for_each(|src| {
                let orig_dist = distances[src];
                let dsts = adj.get(&src).into_iter().flatten();
                let candidate_dists = dsts.map(|&dst| distances[dst].saturating_add(1));
                let new_dist = iter::once(orig_dist).chain(candidate_dists).min().unwrap();
                distances[src] = new_dist;

                is_updated |= new_dist < orig_dist;
            });

            if !is_updated {
                break;
            }
        }

        let dist = distances[0];

        if dist == i32::MAX {
            -1
        } else {
            dist
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p909_test() {
        check(
            vec![
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 35, -1, -1, 13, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 15, -1, -1, -1, -1],
            ],
            4,
        );
        check(vec![vec![-1, -1], vec![-1, 3]], 1);
    }

    fn check(board: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::snakes_and_ladders(board), expect);
    }
}
