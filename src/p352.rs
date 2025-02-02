use std::{cmp, collections::HashMap, ops::RangeInclusive};

const VALUE_MAX: usize = 10000;

pub struct SummaryRanges {
    nodes: Vec<Option<Node>>,
    intervals: HashMap<usize, RangeInclusive<i32>>,
}

impl SummaryRanges {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            nodes: vec![None; VALUE_MAX + 1],
            intervals: HashMap::new(),
        }
    }

    pub fn add_num(&mut self, value: i32) {
        let Self { nodes, intervals } = self;

        let curr = value as usize;

        let curr_node = &mut nodes[curr];
        if curr_node.is_some() {
            return;
        }

        *curr_node = Some(Node {
            parent: curr,
            rank: 0,
        });
        intervals.insert(curr, value..=value);

        let left = curr
            .checked_sub(1)
            .and_then(|left| nodes[left].is_some().then_some(left));
        let right = (curr < VALUE_MAX)
            .then(|| curr + 1)
            .and_then(|right| (nodes[right].is_some()).then_some(right));

        match (left, right) {
            (None, None) => {
                // self.print();
            }
            (None, Some(_right)) => {
                // self.print();
                self.union2(curr);
                // self.print();
            }
            (Some(left), None) => {
                // self.print();
                self.union2(left);
                // self.print();
            }
            (Some(left), Some(_right)) => {
                // self.print();
                self.union2(curr);
                // self.print();
                self.union2(left);
                // self.print();
            }
        }
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut intervals: Vec<_> = self
            .intervals
            .values()
            .map(|int| vec![*int.start(), *int.end()])
            .collect();
        intervals.sort_unstable_by_key(|int| int[0]);
        intervals
    }

    fn find_root(&self, mut value: usize) -> usize {
        loop {
            let node = self.nodes[value].as_ref().unwrap();
            if node.parent == value {
                break value;
            }
            value = node.parent;
        }
    }

    /// Union val and val + 1.
    fn union2(&mut self, val: usize) {
        let lval = val;
        let rval = val + 1;

        let lidx = self.find_root(lval);
        let ridx = self.find_root(rval);

        if lidx == ridx {
            return;
        }

        let Self { nodes, intervals } = self;

        let (lidx, ridx) = if lidx > ridx {
            (ridx, lidx)
        } else {
            (lidx, ridx)
        };

        let (lnode, rnode) = get2(nodes, lidx, ridx);
        let lnode = lnode.as_mut().unwrap();
        let rnode = rnode.as_mut().unwrap();

        let ((lidx, ridx), (lnode, rnode)) = if lnode.rank < rnode.rank {
            ((ridx, lidx), (rnode, lnode))
        } else {
            ((lidx, ridx), (lnode, rnode))
        };

        rnode.parent = lidx;

        if lnode.rank == rnode.rank {
            rnode.rank += 1;
        }

        let rint = intervals.remove(&ridx).unwrap();
        let lint = intervals.get_mut(&lidx).unwrap();

        let new_start = cmp::min(*lint.start(), *rint.start());
        let new_end = cmp::max(*lint.end(), *rint.end());
        *lint = new_start..=new_end;
    }

    // fn print(&self) {
    //     for (value, node) in self.nodes.iter().enumerate() {
    //         if let Some(node) = node {
    //             eprintln!("{value} {node:?}");
    //         }
    //     }
    // }
}

#[derive(Debug, Clone)]
struct Node {
    parent: usize,
    rank: usize,
}

fn get2<T>(slice: &mut [T], lidx: usize, ridx: usize) -> (&mut T, &mut T) {
    debug_assert!(lidx != ridx);
    debug_assert!(lidx < slice.len());
    debug_assert!(ridx < slice.len());

    unsafe {
        let ptr = slice.as_mut_ptr();
        let lptr = ptr.add(lidx);
        let rptr = ptr.add(ridx);
        (lptr.as_mut().unwrap(), rptr.as_mut().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::SummaryRanges;

    #[test]
    fn p352_test() {
        let mut summary_ranges = SummaryRanges::new();

        summary_ranges.add_num(1); // arr = [1]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1]]); // return [[1, 1]]

        summary_ranges.add_num(3); // arr = [1, 3]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 1], vec![3, 3]]); // return [[1, 1], [3, 3]]

        summary_ranges.add_num(7); // arr = [1, 3, 7]
        assert_eq!(
            summary_ranges.get_intervals(),
            vec![vec![1, 1], vec![3, 3], vec![7, 7]]
        ); // return [[1, 1], [3, 3], [7, 7]]

        summary_ranges.add_num(2); // arr = [1, 2, 3, 7]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![7, 7]]); // return [[1, 3], [7, 7]]

        summary_ranges.add_num(6); // arr = [1, 2, 3, 6, 7]
        assert_eq!(summary_ranges.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
        // return [[1, 3], [6, 7]]
    }
}
