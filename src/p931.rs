use crate::Solution;

impl Solution {
    pub fn min_falling_path_sum2(matrix: Vec<Vec<i32>>) -> i32 {
        let mut iter = matrix.into_iter();
        let mut prev = iter.next().unwrap();
        let len = prev.len();

        if len == 1 {
            let sum: i32 = iter.map(|curr| curr[0]).sum();
            return sum + prev[0];
        }

        for mut curr in iter {
            curr[0] += prev[0].min(prev[1]);
            curr[len - 1] += prev[len - 2].min(prev[len - 1]);

            prev.windows(3)
                .map(|win| win[0].min(win[1]).min(win[2]))
                .zip(&mut curr[1..])
                .for_each(|(min, val)| {
                    *val += min;
                });

            prev = curr;
        }

        *prev.iter().min().unwrap()
    }
}
