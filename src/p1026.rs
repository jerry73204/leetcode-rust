use crate::Solution;
use std::cell::RefCell;
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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.unwrap();
        let root = root.borrow();
        let min = root.val;
        let max = root.val;
        let mut max_diff = i32::MIN;

        if let Some(child) = &root.left {
            let child_diff = dfs(&child.borrow(), min, max);
            max_diff = max_diff.max(child_diff);
        }

        if let Some(child) = &root.right {
            let child_diff = dfs(&child.borrow(), min, max);
            max_diff = max_diff.max(child_diff);
        }

        max_diff
    }
}

fn dfs(node: &TreeNode, min: i32, max: i32) -> i32 {
    let mut max_diff = (node.val - min).abs().max((max - node.val).abs());

    let min = min.min(node.val);
    let max = max.max(node.val);

    if let Some(child) = &node.left {
        let child_diff = dfs(&child.borrow(), min, max);
        max_diff = max_diff.max(child_diff);
    }

    if let Some(child) = &node.right {
        let child_diff = dfs(&child.borrow(), min, max);
        max_diff = max_diff.max(child_diff);
    }

    max_diff
}
