use crate::Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut output = vec![];
        let mut next = vec![1];

        for _ in 1..num_rows {
            let prev = next;

            let mid = prev.iter().zip(&prev[1..]).map(|(l, r)| l + r);
            next = Some(1).into_iter().chain(mid).chain(Some(1)).collect();

            output.push(prev);
        }

        output.push(next);
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p118_test() {
        check(
            5,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ],
        );
        check(1, vec![vec![1]]);
    }

    fn check(num_rows: i32, expect: Vec<Vec<i32>>) {
        assert_eq!(Solution::generate(num_rows), expect);
    }
}
