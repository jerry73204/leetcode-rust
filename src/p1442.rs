use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut tab: HashMap<i32, Vec<usize>> = HashMap::with_capacity(len);

        let indices = tab.entry(0).or_insert_with(|| Vec::with_capacity(len));
        indices.push(0);

        let mut xor = 0;
        let mut count = 0;

        for (val, rlen) in arr.into_iter().zip(1..) {
            xor ^= val;
            let indices: &mut Vec<usize> =
                tab.entry(xor).or_insert_with(|| Vec::with_capacity(rlen));

            for &llen in &*indices {
                debug_assert!(llen + 1 < rlen); // because val != 0
                count += rlen - llen - 1;
            }

            indices.push(rlen);
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1442_test() {
        check(vec![2, 3, 1, 6, 7], 4);
        check(vec![1, 1, 1, 1, 1], 10);
        check(vec![1], 0);
        check(vec![1, 1, 1], 2);
    }

    fn check(arr: Vec<i32>, expect: i32) {
        assert_eq!(Solution::count_triplets(arr), expect);
    }
}
