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
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;

        if dfs(&mut root.borrow_mut(), target) {
            None
        } else {
            Some(root)
        }
    }
}

fn dfs(node: &mut TreeNode, target: i32) -> bool {
    if let Some(left) = &node.left {
        if dfs(&mut left.borrow_mut(), target) {
            node.left = None;
        }
    }

    if let Some(right) = &node.right {
        if dfs(&mut right.borrow_mut(), target) {
            node.right = None;
        }
    }

    node.left.is_none() && node.right.is_none() && node.val == target
}
