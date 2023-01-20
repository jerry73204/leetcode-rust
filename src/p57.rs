use crate::Solution;
use std::cmp;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let new_int = new_interval[0]..=new_interval[1];
        let mut output = Vec::with_capacity(intervals.len());
        let mut iter = intervals.into_iter();

        for vec in &mut iter {
            let int = vec[0]..=vec[1];

            if int.end() < new_int.start() {
                output.push(vec);
            } else if int.start() > new_int.end() {
                output.extend([new_interval, vec]);
                output.extend(iter);
                return output;
            } else {
                let mut insert_int = {
                    let start = *cmp::min(int.start(), new_int.start());
                    let end = *cmp::max(int.end(), new_int.end());
                    start..=end
                };

                for vec in &mut iter {
                    let int = vec[0]..=vec[1];

                    if int.start() > insert_int.end() {
                        let insert_vec = vec![*insert_int.start(), *insert_int.end()];
                        output.extend([insert_vec, vec]);
                        output.extend(iter);
                        return output;
                    } else {
                        let start = *insert_int.start();
                        let end = cmp::max(*insert_int.end(), *int.end());
                        insert_int = start..=end;
                    }
                }

                // Reach here if insert_interval merges all following
                // intervals
                let insert_vec = vec![*insert_int.start(), *insert_int.end()];
                output.push(insert_vec);
                return output;
            }
        }

        // Reach here if new_interval comes after all the other
        // intervals
        output.push(new_interval);
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p57_test() {
        check(
            vec![vec![1, 3], vec![6, 9]],
            vec![2, 5],
            vec![vec![1, 5], vec![6, 9]],
        );
        check(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
        );
        check(
            vec![vec![1, 2], vec![5, 8]],
            vec![3, 4],
            vec![vec![1, 2], vec![3, 4], vec![5, 8]],
        );
        check(vec![vec![1, 5]], vec![2, 3], vec![vec![1, 5]]);
    }

    fn check(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>, expect: Vec<Vec<i32>>) {
        assert_eq!(Solution::insert(intervals, new_interval), expect);
    }
}
