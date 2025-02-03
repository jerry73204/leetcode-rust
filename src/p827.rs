use crate::Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let nr = grid.len();
        let nc = grid[0].len();

        let mut island_id_max = 1;
        let mut island_area = vec![0, 0];

        let grid_position_iter = || (0..nr).flat_map(|rix| (0..nc).map(move |cix| [rix, cix]));

        let neighbor_iter = |curr_r: usize, curr_c: usize| {
            let up = curr_r.checked_sub(1).map(|prev_r| [prev_r, curr_c]);
            let down = (curr_r + 1 < nr).then_some([curr_r + 1, curr_c]);
            let left = curr_c.checked_sub(1).map(|prev_c| [curr_r, prev_c]);
            let right = (curr_c + 1 < nc).then_some([curr_r, curr_c + 1]);
            up.into_iter().chain(down).chain(left).chain(right)
        };

        for [rix, cix] in grid_position_iter() {
            match grid[rix][cix] {
                0 => continue,
                1 => {}
                _ => continue,
            }

            island_id_max += 1;
            let island_id = island_id_max;

            let area = {
                let mut area = 0;
                let mut queue = VecDeque::with_capacity(nr * nc);
                queue.push_back([rix, cix]);

                while let Some([curr_r, curr_c]) = queue.pop_front() {
                    let cell = &mut grid[curr_r][curr_c];
                    if *cell != 1 {
                        continue;
                    }

                    *cell = island_id;
                    area += 1;
                    queue.extend(neighbor_iter(curr_r, curr_c));
                }

                area
            };

            island_area.push(area);
        }

        let max_connected_area = grid_position_iter()
            .filter_map(|[curr_r, curr_c]| {
                if grid[curr_r][curr_c] != 0 {
                    return None;
                }

                let neighbors: HashSet<_> = neighbor_iter(curr_r, curr_c)
                    .map(|[r, c]| grid[r][c])
                    .filter(|&island_id| island_id != 0)
                    .collect();

                let sum_area: i32 = neighbors
                    .into_iter()
                    .map(|island_id| island_area[island_id as usize])
                    .sum();

                Some(sum_area + 1)
            })
            .max();

        if let Some(max_connected_area) = max_connected_area {
            max_connected_area
        } else {
            island_area[2..].iter().copied().max().unwrap_or(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3122_test() {
        check(vec![vec![1, 0], vec![0, 1]], 3);
        check(vec![vec![1, 1], vec![0, 1]], 4);
        check(vec![vec![1, 1], vec![1, 1]], 4);
        check(vec![vec![1], vec![1]], 2);
        check(vec![vec![0], vec![1]], 2);
        check(vec![vec![1]], 1);
    }

    fn check(grid: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::largest_island(grid), expect);
    }
}
