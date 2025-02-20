use crate::Solution;
use std::{cmp::Ordering, collections::BTreeMap, ops::RangeInclusive};

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let n = squares.len();

        let min_x = squares.iter().map(|sq| sq[0]).min().unwrap() as u32;
        let max_x = squares.iter().map(|sq| sq[0] + sq[2]).max().unwrap() as u32;

        let mut events: Vec<_> = squares
            .into_iter()
            .flat_map(|sq| {
                let &[x, y, l] = sq.as_slice() else {
                    unreachable!()
                };
                [
                    Event {
                        y,
                        x: x as u32,
                        delta: 1,
                    },
                    Event {
                        y,
                        x: (x + l) as u32,
                        delta: -1,
                    },
                    Event {
                        y: y + l,
                        x: x as u32,
                        delta: -1,
                    },
                    Event {
                        y: y + l,
                        x: (x + l) as u32,
                        delta: 1,
                    },
                ]
            })
            .collect();
        events.sort_unstable();

        let mut counts = FenwickTree::default();
        let mut waterlines = Vec::with_capacity(n);
        let mut prev_y = events[0].y;
        let mut total_area: u64 = 0;

        waterlines.push(Waterline { y: prev_y, area: 0 });

        for event in events {
            let Event { y, x, delta } = event;
            // eprintln!("y={y} x={x} d={delta}");

            if y != prev_y {
                let height = y - prev_y;
                // dbg!(counts.range(min_x..=max_x).collect::<Vec<_>>());
                let width: u32 = counts
                    .range(min_x..=max_x)
                    .scan(None, |start_at, (x, count)| {
                        let len = match (*start_at, count) {
                            (None, 0) => None,
                            (None, _) => {
                                *start_at = Some(x);
                                None
                            }
                            (Some(s), 0) => {
                                *start_at = None;
                                Some(x - s)
                            }
                            (Some(_), _) => None,
                        };
                        Some(len)
                    })
                    .flatten()
                    .sum();
                total_area += width as u64 * height as u64;
                waterlines.push(Waterline {
                    y,
                    area: total_area,
                });
                // eprintln!(" y={y} area={total_area}");
                prev_y = y;
            }

            counts.update(x, delta);
        }

        let half_area = total_area as f64 / 2.0;
        let result = waterlines.binary_search_by_key(&OrdF64(half_area), |waterline| {
            OrdF64(waterline.area as f64)
        });

        // dbg!(&waterlines);

        let separator_y = match result {
            Ok(idx) => {
                let prev = idx.checked_sub(1).map(|prev_idx| waterlines[prev_idx]);
                let curr = &waterlines[idx];
                let y = if let Some(prev) = prev {
                    if prev.area == curr.area {
                        prev.y
                    } else {
                        curr.y
                    }
                } else {
                    curr.y
                };
                y as f64
            }
            Err(idx) => {
                let prev = &waterlines[idx - 1];
                let next = &waterlines[idx];
                let step_area = (next.area - prev.area) as f64;
                let w0 = (next.area as f64 - half_area) / step_area;
                let w1 = (half_area - prev.area as f64) / step_area;
                prev.y as f64 * w0 + next.y as f64 * w1
            }
        };

        separator_y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct OrdF64(f64);

impl Eq for OrdF64 {}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Waterline {
    y: i32,
    area: u64,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Event {
    y: i32,
    x: u32,
    delta: i32,
}

#[derive(Debug, Clone, Default)]
struct FenwickTree {
    nodes: BTreeMap<u32, i32>,
}

impl FenwickTree {
    pub fn update(&mut self, index: u32, delta: i32) {
        // eprintln!(" update() index={index} value={value}");
        if delta == 0 {
            return;
        }

        for curr in u32_upper_bounds(index) {
            // eprintln!(" {curr:034b}");
            let value = self.nodes.entry(curr).or_default();
            *value += delta;

            if *value == 0 {
                self.nodes.remove(&curr);
            }
        }
    }

    pub fn range(&self, range: RangeInclusive<u32>) -> impl Iterator<Item = (u32, i32)> + '_ {
        let start = *range.start();
        let end = *range.end();
        let inner_range = (start + 1)..end;

        let mut stack = Vec::with_capacity(u32::BITS as usize + 1);
        if let Some(&value) = self.nodes.get(&0) {
            stack.push((0, value));
        }

        let index_iter = {
            let prefix = [start];
            let inner = self.nodes.range(inner_range).map(|(&index, _)| index);
            let suffix = [end];
            prefix.into_iter().chain(inner).chain(suffix)
        };

        let mut prev = 0;

        index_iter.flat_map(move |curr| {
            let mut yields = vec![];

            let mut push_yield = |ix: u32, sum: i32| {
                while let Some(&(last, _)) = yields.last() {
                    if last < ix {
                        break;
                    }
                    yields.pop();
                }
                yields.push((ix, sum));
            };

            let prefix = u32_common_prefix(prev, curr);

            while let Some(&(last, _)) = stack.last() {
                if last <= prefix {
                    break;
                }
                stack.pop();

                if let Some(upper) = u32_upper_bound(last) {
                    let sum = match stack.last() {
                        Some(&(_, sum)) => sum,
                        None => 0,
                    };
                    push_yield(upper, sum);
                }
            }

            let append: Vec<_> = u32_lower_bounds(curr)
                .take_while(|&lower| lower > prefix)
                .filter_map(|lower| Some((lower, *self.nodes.get(&lower)?)))
                .collect();
            for (lower, value) in append.into_iter().rev() {
                let last_sum = match stack.last() {
                    Some(&(_, sum)) => sum,
                    None => 0,
                };
                stack.push((lower, last_sum + value));
            }

            let curr_sum = match stack.last() {
                Some(&(_, sum)) => sum,
                None => 0,
            };
            prev = curr;
            push_yield(curr, curr_sum);

            yields
        })
    }
}

fn u32_upper_bound(index: u32) -> Option<u32> {
    index.checked_add(lsb(index)?)
}

fn u32_lower_bound(index: u32) -> Option<u32> {
    Some(index - lsb(index)?)
}

fn u32_upper_bounds(index: u32) -> impl Iterator<Item = u32> {
    std::iter::successors(Some(index), |&prev| u32_upper_bound(prev))
}

fn u32_lower_bounds(index: u32) -> impl Iterator<Item = u32> {
    std::iter::successors(Some(index), |&prev| u32_lower_bound(prev))
}

fn u32_common_prefix(lhs: u32, rhs: u32) -> u32 {
    let nz = (lhs ^ rhs).leading_zeros();
    let bit = 1u32 << (u32::BITS - nz);
    let mask = !(bit).wrapping_sub(1);
    lhs & mask
}

fn lsb(val: u32) -> Option<u32> {
    if val == 0 {
        return None;
    }
    Some(1 << val.trailing_zeros())
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;
    use crate::Solution;

    #[test]
    fn p3454_test() {
        check(&[[0, 0, 1], [2, 2, 1]], 1.0);
        check(&[[0, 0, 2], [1, 1, 1]], 1.0);
        check(&[[16, 27, 3], [18, 24, 5]], 27.14286);
        check(&[[15, 21, 2], [19, 21, 3]], 22.3);
        check(&[[11, 13, 1], [9, 21, 5]], 23.4);
        check(
            &[
                [999892931, 999974790, 6788622],
                [319710671, 963660807, 5518783],
                [623736653, 934759633, 4248549],
                [234214719, 848813522, 417010],
                [154771654, 645515409, 9370045],
                [965571354, 998982755, 10809560],
                [338822522, 550588284, 12471651],
                [168193362, 682286828, 5173004],
                [459856474, 778674604, 5635628],
                [806653114, 860720237, 1444683],
            ],
            685078297.72824,
        );
    }

    fn check(squares: &[[i32; 3]], expect: f64) {
        let squares: Vec<_> = squares.iter().map(|s| s.to_vec()).collect();
        let answer = Solution::separate_squares(squares);
        assert!(
            (answer - expect).abs() <= 1e-5,
            "expect {expect}, but get {answer}"
        );
    }

    #[test]
    fn p3454_fenwick_tree_test() {
        let mut tree = FenwickTree::default();
        tree.update(2, 1);
        tree.update(4, -1);

        let sums: Vec<_> = tree.range(0..=4).collect();
        assert_eq!(sums, [(0, 0), (2, 1), (4, 0)]);
    }
}
