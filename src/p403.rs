use crate::Solution;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[0..2] != [0, 1] {
            return false;
        }
        let stones = &stones[1..];

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct State {
            pub index: usize,
            pub step: i32,
        }

        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(State { index: 0, step: 1 }));

        while let Some(Reverse(state)) = heap.pop() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            let State { index, step } = state;
            let pos = stones[index];
            // eprintln!("i={index}\tp={pos}\ts={step}");

            if index == stones.len() - 1 {
                return true;
            }

            let next_pos1 = pos + step - 1;
            let next_pos2 = next_pos1 + 1;
            let next_pos3 = next_pos2 + 1;

            let trail = &stones[index..];
            match trail.binary_search(&next_pos1) {
                Ok(offset) => {
                    let next_index1 = index + offset;

                    if offset != 0 {
                        heap.push(Reverse(State {
                            index: next_index1,
                            step: step - 1,
                        }))
                    };

                    let guess_index = stones.get(next_index1 + 1);

                    if guess_index == Some(&next_pos2) {
                        let next_index2 = next_index1 + 1;
                        heap.push(Reverse(State {
                            index: next_index2,
                            step,
                        }));

                        if stones.get(next_index2 + 1) == Some(&next_pos3) {
                            let next_index3 = next_index2 + 1;
                            heap.push(Reverse(State {
                                index: next_index3,
                                step: step + 1,
                            }));
                        }
                    } else if guess_index == Some(&next_pos3) {
                        let next_index3 = next_index1 + 1;

                        heap.push(Reverse(State {
                            index: next_index3,
                            step: step + 1,
                        }));
                    }
                }
                Err(offset) => {
                    let checked_index = index + offset;
                    let checked_pos = match stones.get(checked_index) {
                        Some(&checked_pos) => checked_pos,
                        None => continue,
                    };

                    if checked_pos == next_pos2 {
                        let next_index2 = checked_index;
                        heap.push(Reverse(State {
                            index: next_index2,
                            step,
                        }));

                        if stones.get(next_index2 + 1) == Some(&next_pos3) {
                            let next_index3 = next_index2 + 1;
                            heap.push(Reverse(State {
                                index: next_index3,
                                step: step + 1,
                            }));
                        }
                    } else if checked_pos == next_pos3 {
                        let next_index3 = checked_index;
                        heap.push(Reverse(State {
                            index: next_index3,
                            step: step + 1,
                        }));
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p403_test() {
        check(vec![0, 1, 3, 5, 6, 8, 12, 17], true);
        check(vec![0, 1, 2, 3, 4, 8, 9, 11], false);
        check(vec![0, 2], false);
    }

    fn check(stones: Vec<i32>, expect: bool) {
        assert_eq!(Solution::can_cross(stones), expect);
    }
}
