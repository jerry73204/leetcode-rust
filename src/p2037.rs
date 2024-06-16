use crate::Solution;

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut marks: Vec<i32> = seats
            .into_iter()
            .zip(students)
            .flat_map(|(seat, student)| [seat, -student])
            .collect();
        marks.sort_unstable_by_key(|val| (val.abs(), val.is_positive()));

        let mut iter = marks.into_iter();
        let first = iter.next().unwrap();

        let (total, _, _) = iter.fold(
            (0, first.abs(), first.signum()),
            |(total, prev, count), val| {
                let curr = val.abs();
                let total = total + count.abs() * (curr - prev);
                let count = count + val.signum();
                (total, curr, count)
            },
        );

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2037_test() {
        check(vec![3, 1, 5], vec![2, 7, 4], 4);
        check(vec![4, 1, 5, 9], vec![1, 3, 2, 6], 7);
        check(vec![2, 2, 6, 6], vec![1, 3, 2, 6], 4);
    }

    fn check(seats: Vec<i32>, students: Vec<i32>, expect: i32) {
        assert_eq!(Solution::min_moves_to_seat(seats, students), expect);
    }
}
