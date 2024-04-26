use crate::Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let nr = grid.len();
        let nc = grid[0].len();

        let init = [0; 10];

        let changes = (0..nc).fold(init, |prev, ic| {
            let mut counts = [0; 10];

            grid.iter().for_each(|row| {
                let val = row[ic];
                counts[val as usize] += 1;
            });

            dbg!(ic, &counts);

            let (min1_val, min1_changes) = prev
                .iter()
                .enumerate()
                .min_by_key(|&(_, changes)| changes)
                .unwrap();
            let (_, min2_changes) = prev
                .iter()
                .enumerate()
                .filter(|&(val, _)| val != min1_val)
                .min_by_key(|&(_, changes)| changes)
                .unwrap();

            let mut curr = [0; 10];

            counts.iter().enumerate().for_each(|(val, cnt)| {
                let prev_changes = if val != min1_val {
                    min1_changes
                } else {
                    min2_changes
                };

                let curr_changes = prev_changes + (nr - cnt);
                curr[val] = curr_changes;
            });

            curr
        });

        let (_, &changes) = changes
            .iter()
            .enumerate()
            .min_by_key(|&(_, changes)| changes)
            .unwrap();

        changes as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3122_test() {
        check(vec![vec![1, 0, 2], vec![1, 0, 2]], 0);
        check(vec![vec![1, 1, 1], vec![0, 0, 0]], 3);
        check(vec![vec![1], vec![2], vec![3]], 2);
        check(vec![vec![1]], 0);
        check(vec![vec![1, 2, 3]], 0);
        check(vec![vec![3, 3, 3]], 1);
    }

    fn check(grid: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::minimum_operations(grid), expect);
    }
}
