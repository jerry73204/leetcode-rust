use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }

        let mut steps = vec![0; (n + 1) as usize];
        steps[0] = 1;
        steps[1] = 1;

        for idx in 2..=(n as usize) {
            steps[idx] = steps[idx - 1] + steps[idx - 2];
        }

        steps[n as usize]
    }
}
