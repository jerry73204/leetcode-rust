use crate::Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut marks: Vec<_> = (0..points.len())
            .flat_map(|id| [Mark { id, is_end: false }, Mark { id, is_end: true }])
            .collect();

        marks.sort_by_cached_key(|mark| {
            let int = &points[mark.id];
            let pos = if mark.is_end { int[1] } else { int[0] };
            (pos, mark.is_end)
        });

        let mut count = 0;
        let mut start = None;

        for mark in marks {
            if mark.is_end {
                let int = &points[mark.id];

                if let Some(start_) = start {
                    if int[0] >= start_ {
                        start = None;
                        count += 1;
                    }
                }
            } else if start.is_none() {
                let int = &points[mark.id];
                start = Some(int[0]);
            }
        }

        count
    }
}

struct Mark {
    pub id: usize,
    pub is_end: bool,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p452_test() {
        check(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]], 2);
        check(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], 4);
        check(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], 2);
    }

    fn check(points: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::find_min_arrow_shots(points), expect);
    }
}
