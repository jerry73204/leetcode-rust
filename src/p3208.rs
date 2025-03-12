use crate::Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = colors.len();

        // A closure to check if colors[ix] and its right have
        // distinct colors.
        let is_alternating = |ix: usize| colors[ix] != colors[(ix + 1) % n];

        // Count number of k-groups within a n-group.
        let count_k_groups = |len: usize| match len.checked_sub(k) {
            Some(sub) => sub + 1,
            None => 0,
        };

        // Find the first occurrence of ix such that colors[ix] !=
        // colors[ix + 1]
        let Some(first_eq) = (0..n).find(|&ix| !is_alternating(ix)) else {
            let count = if n >= k { n } else { 0 };
            return count as i32;
        };

        // Create an iterator that enumerate the ix that colors[ix] !=
        // colors[ix + 1].
        let mut ix_iter = {
            let start = (first_eq + 1) % n;
            (start..n).chain(0..start).filter(|&ix| !is_alternating(ix))
        };

        // Find the second occurrence. If the 1st and the 2nd are in
        // the same position, it goes through a cycle and goes back to
        // the first occurrence.
        let second_eq = ix_iter.next().unwrap();
        if first_eq == second_eq {
            return count_k_groups(n) as i32;
        }

        // If not, count number of k-groups.
        let mut count = count_k_groups((second_eq + n - first_eq) % n);
        let mut last_eq = second_eq;

        for ix in ix_iter {
            let group_len = (ix + n - last_eq) % n;
            count += count_k_groups(group_len);
            last_eq = ix;
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p3208_test() {
        // check(&[0, 1, 0, 1, 0], 3, 3);
        // check(&[0, 1, 0, 1], 3, 4);
        check(&[0, 0, 0, 1], 3, 1);
    }

    fn check(colors: &[i32], k: i32, expect: i32) {
        assert_eq!(
            Solution::number_of_alternating_groups(colors.to_vec(), k),
            expect
        );
    }
}
