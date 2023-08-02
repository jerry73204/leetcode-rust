use crate::Solution;
use std::iter;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut outputs = Vec::with_capacity(estimate_size(n as usize, k as usize));

        recursion(
            &mut outputs,
            &mut vec![0; k as usize],
            0,
            n as usize,
            k as usize,
        );
        outputs
    }
}

fn estimate_size(n: usize, k: usize) -> usize {
    let (num, deno) = iter::successors(Some((n, k)), |(n, k)| (*k > 1).then(|| (n - 1, k - 1)))
        .fold((1, 1), |(num, deno), (n, k)| (num * n, deno * k));
    num / deno
}

fn recursion(outputs: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>, taken: usize, n: usize, k: usize) {
    if k == 0 {
        outputs.push(buf.clone());
        return;
    }

    let remain = n - k;
    (0..=remain).for_each(|take| {
        buf[k - 1] = (taken + take + 1) as i32;
        recursion(outputs, buf, taken + take + 1, n - take - 1, k - 1);
    });
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p77_test() {
        check(1, 1, vec![vec![1]]);
        check(3, 1, vec![vec![1], vec![2], vec![3]]);
        check(3, 2, vec![vec![2, 1], vec![3, 1], vec![3, 2]]);
        check(
            4,
            2,
            vec![
                vec![2, 1],
                vec![3, 1],
                vec![4, 1],
                vec![3, 2],
                vec![4, 2],
                vec![4, 3],
            ],
        );
        check(
            5,
            3,
            vec![
                vec![3, 2, 1],
                vec![4, 2, 1],
                vec![5, 2, 1],
                vec![4, 3, 1],
                vec![5, 3, 1],
                vec![5, 4, 1],
                vec![4, 3, 2],
                vec![5, 3, 2],
                vec![5, 4, 2],
                vec![5, 4, 3],
            ],
        );
    }

    fn check(n: i32, k: i32, expect: Vec<Vec<i32>>) {
        assert_eq!(Solution::combine(n, k), expect);
    }
}
