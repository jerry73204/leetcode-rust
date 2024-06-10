use crate::Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort_unstable();
        expected
            .into_iter()
            .zip(heights)
            .filter(|(expect, orig)| expect != orig)
            .count() as i32
    }
}
