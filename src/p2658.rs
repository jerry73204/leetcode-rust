use crate::Solution;

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        let nr = grid.len();
        let nc = grid[0].len();

        let neighbor_iter = |cr: usize, cc: usize| {
            let up = cr.checked_sub(1).map(|pr| [pr, cc]);
            let down = (cr + 1 < nr).then_some([cr + 1, cc]);
            let left = cc.checked_sub(1).map(|pr| [cr, pr]);
            let right = (cc + 1 < nc).then_some([cr, cc + 1]);
            up.into_iter().chain(down).chain(left).chain(right)
        };

        let mut max_fish = 0;

        for start_r in 0..nr {
            for start_c in 0..nc {
                let start_cell = &mut grid[start_r][start_c];

                if *start_cell == 0 {
                    continue;
                }

                let mut n_fish = 0;
                let mut queue = Vec::with_capacity(nr * nc);
                queue.push([start_r, start_c]);

                while let Some([curr_r, curr_c]) = queue.pop() {
                    let curr_cell = &mut grid[curr_r][curr_c];
                    if *curr_cell == 0 {
                        continue;
                    }
                    n_fish += *curr_cell;
                    *curr_cell = 0;
                    queue.extend(neighbor_iter(curr_r, curr_c));
                }

                max_fish = max_fish.max(n_fish);
            }
        }

        max_fish
    }
}
