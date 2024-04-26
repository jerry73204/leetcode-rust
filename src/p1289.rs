use crate::Solution;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 {
            return grid[0][0];
        }

        let mins = (0, 0, 0);

        let (_, min_sum, _) = grid.into_iter().fold(mins, |prev_mins, row| {
            let (min_ix, min_val1, min_val2) = prev_mins;

            let mut iter = row.into_iter().enumerate().map(|(ix, val)| {
                let prev_min = if ix == min_ix { min_val2 } else { min_val1 };
                let min_at_ix = val + prev_min;
                (ix, min_at_ix)
            });
            let (i1, v1) = iter.next().unwrap();
            let (i2, v2) = iter.next().unwrap();

            let curr_mins = if v1 < v2 { (i1, v1, v2) } else { (i2, v2, v1) };
            let curr_mins = iter.fold(curr_mins, |(min_i, min1, min2), (ix, val)| {
                if val < min1 {
                    (ix, val, min1)
                } else if val < min2 {
                    (min_i, min1, val)
                } else {
                    (min_i, min1, min2)
                }
            });

            dbg!(curr_mins);

            curr_mins
        });

        min_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1289_test() {
        check(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13);
        check(vec![vec![1]], 1);
        check(vec![vec![1, 2], vec![4, 6]], 6);
    }

    fn check(grid: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::min_falling_path_sum(grid), expect);
    }
}
