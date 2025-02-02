use crate::Solution;
use std::collections::BTreeSet;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let get_height = |index: usize| -> i32 { buildings[index][2] };

        let mut marks: Vec<Mark> = buildings
            .iter()
            .enumerate()
            .flat_map(|(index, range)| {
                let (left, right) = match range.as_slice() {
                    &[left, right, _] => (left, right),
                    _ => unreachable!(),
                };

                [
                    Mark {
                        pos: left,
                        is_end: false,
                        index,
                    },
                    Mark {
                        pos: right,
                        is_end: true,
                        index,
                    },
                ]
            })
            .collect();
        marks.sort_unstable();

        let mut set = BTreeSet::new();
        let mut iter = marks.into_iter();
        let mut output = vec![];

        let mut prev_height = 0;
        let mut saved = {
            let Mark { pos, index, .. } = iter.next().unwrap();
            set.insert(Entry {
                height: get_height(index),
                index,
            });
            Event {
                pos,
                height: get_height(index),
            }
        };

        for mark in iter {
            let entry = Entry {
                height: get_height(mark.index),
                index: mark.index,
            };

            if mark.is_end {
                set.remove(&entry);
            } else {
                set.insert(entry);
            }

            let max_height = set
                .iter()
                .next_back()
                .map(|entry| entry.height)
                .unwrap_or(0);

            if mark.pos != saved.pos {
                if prev_height != saved.height {
                    output.push(vec![saved.pos, saved.height]);
                    prev_height = saved.height;
                }
                saved = Event {
                    pos: mark.pos,
                    height: max_height,
                };
            } else {
                saved.height = max_height;
            }
        }

        if prev_height != saved.height {
            output.push(vec![saved.pos, saved.height]);
        }

        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Mark {
    pub pos: i32,
    pub is_end: bool,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    pub height: i32,
    pub index: usize,
}

#[derive(Debug, Clone)]
struct Event {
    pub pos: i32,
    pub height: i32,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p218_test() {
        check(
            vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8],
            ],
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0],
            ],
        );
        check(
            vec![vec![0, 2, 3], vec![2, 5, 3]],
            vec![vec![0, 3], vec![5, 0]],
        );
        check(
            vec![vec![1, 2, 1], vec![1, 2, 2], vec![1, 2, 3]],
            vec![vec![1, 3], vec![2, 0]],
        );
        check(
            vec![vec![2, 4, 7], vec![2, 4, 5], vec![2, 4, 6]],
            vec![vec![2, 7], vec![4, 0]],
        );
    }

    fn check(buildings: Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        assert_eq!(Solution::get_skyline(buildings), expect);
    }
}
