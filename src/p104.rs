use crate::Solution;
use std::cell::RefCell;
use std::cmp;
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = match root {
            Some(root) => root,
            None => return 0,
        };

        let mut stack = vec![(root, 1)];
        let mut max_depth = 0;

        while let Some((node, depth)) = stack.pop() {
            let node = node.borrow();
            max_depth = cmp::max(max_depth, depth);

            if let Some(child) = &node.right {
                stack.push((child.clone(), depth + 1));
            }
            if let Some(child) = &node.left {
                stack.push((child.clone(), depth + 1));
            }
        }

        max_depth
    }
}
