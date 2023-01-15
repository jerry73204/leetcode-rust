use crate::Solution;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut marks: Vec<_> = intervals
            .into_iter()
            .flat_map(|int| {
                vec![
                    Mark {
                        value: int[0],
                        is_end: false,
                    },
                    Mark {
                        value: int[1],
                        is_end: true,
                    },
                ]
            })
            .collect();
        marks.sort_unstable();

        let mut last_start = 0;
        let mut interval_count = 0usize;
        let mut merged_intervals = vec![];

        for mark in marks {
            if mark.is_end {
                interval_count -= 1;

                if interval_count == 0 {
                    merged_intervals.push(vec![last_start, mark.value]);
                }
            } else {
                if interval_count == 0 {
                    last_start = mark.value;
                }
                interval_count += 1;
            }
        }

        merged_intervals
    }
}

pub struct Mark {
    pub value: i32,
    pub is_end: bool,
}

impl PartialOrd for Mark {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(Equal) => {}
            ord => return ord,
        }
        self.is_end.partial_cmp(&other.is_end)
    }
}

impl Ord for Mark {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Equal => {}
            ord => return ord,
        }
        self.is_end.cmp(&other.is_end)
    }
}

impl PartialEq for Mark {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.is_end == other.is_end
    }
}

impl Eq for Mark {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p56_test() {
        check(
            vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        );
        check(vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]);
    }

    fn check(intervals: Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        assert_eq!(Solution::merge(intervals), expect);
    }
}
