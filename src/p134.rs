use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut runs: VecDeque<_> = gas
            .into_iter()
            .zip(cost)
            .enumerate()
            .map(|(station, (gas, cost))| Run {
                station,
                cost: gas - cost,
            })
            .collect();

        debug_assert!(!runs.is_empty());

        loop {
            // Merge consecutive positive and negative runs
            {
                let mut acc: Option<Run> = None;

                for _ in 0..runs.len() {
                    let run = runs.pop_front().unwrap();

                    match acc.take() {
                        Some(mut acc_run) => {
                            if (acc_run.cost >= 0) == (run.cost >= 0) {
                                acc_run.cost += run.cost;
                                acc = Some(acc_run);
                            } else {
                                runs.push_back(acc_run);
                                acc = Some(run);
                            }
                        }
                        None => acc = Some(run),
                    }
                }

                if let Some(acc_run) = acc {
                    runs.push_back(acc_run);
                }
            }

            if runs.len() == 1 {
                if runs[0].cost >= 0 {
                    return runs[0].station as i32;
                } else {
                    return -1;
                }
            }

            // If the costs of first and last runs have the same sign,
            // merge them into one run.
            if runs.len() % 2 == 1 {
                if runs[0].cost >= 0 {
                    let last = runs.pop_back().unwrap();
                    runs[0] = Run {
                        station: last.station,
                        cost: runs[0].cost + last.cost,
                    };
                } else {
                    let first = runs.pop_front().unwrap();
                    runs.back_mut().unwrap().cost += first.cost;
                }
            } else if runs[0].cost < 0 {
                runs.rotate_left(1);
            }

            // Merge neighboring positive and negative runs.
            {
                debug_assert!(runs.len() % 2 == 0);
                for _ in 0..(runs.len() / 2) {
                    let pos_run = runs.pop_front().unwrap();
                    let neg_run = runs.pop_front().unwrap();
                    debug_assert!(pos_run.cost >= 0);
                    debug_assert!(neg_run.cost < 0);

                    let new_run = Run {
                        station: pos_run.station,
                        cost: pos_run.cost + neg_run.cost,
                    };

                    runs.push_back(new_run);
                }
            }
        }
    }
}

struct Run {
    pub station: usize,
    pub cost: i32,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p134_test() {
        check(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3);
        check(vec![2, 3, 4], vec![3, 4, 3], -1);
        check(vec![1], vec![1], 0);
        check(vec![1], vec![2], -1);
        check(vec![1, 2], vec![2, 1], 1);
        check(vec![2, 1], vec![1, 2], 0);
    }

    fn check(gas: Vec<i32>, cost: Vec<i32>, expect: i32) {
        assert_eq!(Solution::can_complete_circuit(gas, cost), expect);
    }
}
