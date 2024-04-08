use crate::Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut num_0: usize = students.iter().filter(|&&s| s == 0).count();
        let mut num_1: usize = students.len() - num_0;

        for val in sandwiches {
            match val {
                0 => {
                    if let Some(num_0_) = num_0.checked_sub(1) {
                        num_0 = num_0_;
                    } else {
                        break;
                    }
                }
                1 => {
                    if let Some(num_1_) = num_1.checked_sub(1) {
                        num_1 = num_1_;
                    } else {
                        break;
                    }
                }
                _ => {}
            }
        }

        (num_0 + num_1) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1700_test() {
        check(vec![1, 1, 0, 0], vec![0, 1, 0, 1], 0);
        check(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1], 3);
    }

    fn check(students: Vec<i32>, sandwiches: Vec<i32>, expect: i32) {
        assert_eq!(Solution::count_students(students, sandwiches), expect);
    }
}
