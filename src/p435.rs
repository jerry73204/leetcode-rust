use crate::Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        // Sort intervals by the end and then by the start of the interval.
        let mut intervals: Vec<_> = intervals.into_iter().map(|int| int[0]..int[1]).collect();
        intervals.sort_unstable_by_key(|int| int.end);

        let mut iter = intervals.into_iter();
        let first = iter.next().unwrap();
        let (_, count) = iter.fold((first.end, 0), |(reach, count), int| {
            if reach <= int.start {
                (int.end, count)
            } else {
                (reach, count + 1)
            }
        });

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p435_test() {
        check(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]], 1);
        check(vec![vec![1, 2], vec![1, 2], vec![1, 2]], 2);
        check(vec![vec![1, 2], vec![2, 3]], 0);
        check(
            vec![
                vec![-52, 31],
                vec![-73, -26],
                vec![82, 97],
                vec![-65, -11],
                vec![-62, -49],
                vec![95, 99],
                vec![58, 95],
                vec![-31, 49],
                vec![66, 98],
                vec![-63, 2],
                vec![30, 47],
                vec![-40, -26],
            ],
            7,
        );
    }

    fn check(intervals: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::erase_overlap_intervals(intervals), expect);
    }
}
