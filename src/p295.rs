#![allow(dead_code)]

use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    left_heap: BinaryHeap<i32>,
    medians: (Option<i32>, Option<i32>),
    right_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left_heap: BinaryHeap::new(),
            medians: (None, None),
            right_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.medians = match self.medians {
            (None, None) => (Some(num), None),
            (None, Some(_)) => unreachable!(),
            (Some(lval), None) => {
                if num < lval {
                    let left_heap = &mut self.left_heap;
                    left_heap.push(num);
                    let new_lval = left_heap.pop().unwrap();
                    (Some(new_lval), Some(lval))
                } else {
                    let right_heap = &mut self.right_heap;
                    right_heap.push(Reverse(num));
                    let Reverse(new_rval) = right_heap.pop().unwrap();
                    (Some(lval), Some(new_rval))
                }
            }
            (Some(lval), Some(rval)) => {
                if num < lval {
                    self.left_heap.push(num);
                    self.right_heap.push(Reverse(rval));
                    (Some(lval), None)
                } else if num > rval {
                    self.left_heap.push(lval);
                    self.right_heap.push(Reverse(num));
                    (Some(rval), None)
                } else {
                    self.left_heap.push(lval);
                    self.right_heap.push(Reverse(rval));
                    (Some(num), None)
                }
            }
        };
    }

    fn find_median(&self) -> f64 {
        match self.medians {
            (None, _) => unreachable!(),
            (Some(median), None) => median as f64,
            (Some(lval), Some(rval)) => ((lval + rval) as f64) / 2.0,
        }
    }
}
