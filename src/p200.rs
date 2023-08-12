use crate::Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let nrow = grid.len();
        let ncol = grid[0].len();

        let mut visited = vec![vec![false; ncol]; nrow];
        let mut island_count = 0;

        for r in 0..nrow {
            for c in 0..ncol {
                if visited[r][c] {
                    continue;
                }

                if grid[r][c] == '1' {
                    explore((r, c), &grid, &mut visited);
                    island_count += 1;
                }
            }
        }

        island_count
    }
}

fn explore(start_pos: (usize, usize), grid: &[Vec<char>], visited: &mut [Vec<bool>]) {
    let nrow = grid.len();
    let ncol = grid[0].len();

    let get_neighbors = |r: usize, c: usize| {
        debug_assert_eq!(grid[r][c], '1');

        let left = r.checked_sub(1).map(|r_sub| (r_sub, c));
        let top = c.checked_sub(1).map(|c_sub| (r, c_sub));
        let right = (r + 1 < nrow).then(|| (r + 1, c));
        let bottom = (c + 1 < ncol).then(|| (r, c + 1));

        let grid = &grid;
        left.into_iter()
            .chain(top)
            .chain(right)
            .chain(bottom)
            .filter(move |&(r, c)| grid[r][c] == '1')
    };

    let mut stack = vec![start_pos];

    while let Some((r, c)) = stack.pop() {
        let vis = &mut visited[r][c];

        if *vis {
            continue;
        }

        *vis = true;
        stack.extend(get_neighbors(r, c));
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p200_test() {
        check(
            vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ],
            1,
        );
        check(
            vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ],
            3,
        );
    }

    fn check(grid: Vec<Vec<char>>, expect: i32) {
        assert_eq!(Solution::num_islands(grid), expect);
    }
}
