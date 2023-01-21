use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let diff_count: HashMap<i32, i32> =
            tasks
                .into_iter()
                .fold(HashMap::new(), |mut diff_count, diff| {
                    diff_count
                        .entry(diff)
                        .and_modify(|count| {
                            *count += 1;
                        })
                        .or_insert(1);
                    diff_count
                });

        let mut total_rounds = 0;

        for (_, count) in diff_count {
            let quot = count / 3;
            let rem = count % 3;

            let rounds = match rem {
                0 => quot,
                1 => {
                    if quot == 0 {
                        return -1;
                    } else {
                        quot + 1
                    }
                }
                2 => quot + 1,
                _ => unreachable!(),
            };

            total_rounds += rounds;
        }

        total_rounds
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2244_test() {
        check(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4], 4);
        check(vec![2, 3, 3], -1);
    }

    fn check(tasks: Vec<i32>, expect: i32) {
        assert_eq!(Solution::minimum_rounds(tasks), expect);
    }
}
