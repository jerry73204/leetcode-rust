use crate::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        find(&nums)
    }
}

fn find(slice: &[i32]) -> i32 {
    if !is_rotated(slice) {
        return slice[0];
    }

    let len = slice.len();
    let half_len = len / 2;
    let (lslice, rslice) = slice.split_at(half_len);

    match (is_rotated(lslice), is_rotated(rslice)) {
        (true, true) => unreachable!(),
        (true, false) => find(lslice),
        (false, true) => find(rslice),
        (false, false) => rslice[0],
    }
}

fn is_rotated(slice: &[i32]) -> bool {
    let first = *slice.first().unwrap();
    let last = *slice.last().unwrap();
    first > last
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p153_test() {
        check(vec![3, 4, 5, 1, 2], 1);
        check(vec![4, 5, 6, 7, 0, 1, 2], 0);
        check(vec![11, 13, 15, 17], 11);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::find_min(nums), expect);
    }
}
