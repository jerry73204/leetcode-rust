use crate::Solution;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        macro_rules! ptr {
            ($node:expr) => {
                Rc::as_ptr($node)
            };
        }
        let to_char = |val: i32| (b'a' + val as u8) as char;

        let (parent_tab, mut frontiers) = {
            let mut leaves = VecDeque::new();
            let mut tab = HashMap::new();
            let mut stack = vec![];

            macro_rules! process {
                ($node:expr) => {
                    let borrow = $node.borrow();

                    if let Some(left) = &borrow.left {
                        stack.push((left.clone(), $node.clone()));
                    }

                    if let Some(right) = &borrow.right {
                        stack.push((right.clone(), $node.clone()));
                    }

                    if borrow.left.is_none() && borrow.right.is_none() {
                        leaves.push_back($node.clone());
                    }
                };
            }

            if let Some(root) = &root {
                process!(root);
            }

            while let Some((curr, parent)) = stack.pop() {
                tab.insert(ptr!(&curr), parent);
                process!(curr);
            }

            (tab, leaves)
        };

        let mut output = String::new();
        loop {
            let min_val = frontiers
                .iter()
                .map(|node| node.borrow().val)
                .min()
                .unwrap();
            output.push(to_char(min_val));

            let n_children = frontiers.len();
            let mut visited = HashSet::new();

            for _ in 0..n_children {
                let curr = frontiers.pop_front().unwrap();

                if curr.borrow().val != min_val {
                    continue;
                }

                let parent = match parent_tab.get(&ptr!(&curr)) {
                    Some(parent) => parent,
                    None => return output,
                };

                if !visited.insert(ptr!(parent)) {
                    continue;
                }

                frontiers.push_back(parent.clone());
            }
        }
    }
}
