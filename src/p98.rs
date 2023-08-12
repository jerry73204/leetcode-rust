use crate::Solution;
use std::cell::RefCell;
use std::ops::{Bound, RangeBounds};
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        dfs(root.as_ref(), (Bound::Unbounded, Bound::Unbounded))
    }
}

fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, bound: (Bound<i32>, Bound<i32>)) -> bool {
    let node = match node {
        Some(node) => node,
        None => return true,
    };
    let node = node.borrow();

    if !bound.contains(&node.val) {
        return false;
    }

    let lchild = node.left.as_ref();
    let rchild = node.right.as_ref();

    dfs(lchild, (bound.0, Bound::Excluded(node.val)))
        && dfs(rchild, (Bound::Excluded(node.val), bound.1))
}
