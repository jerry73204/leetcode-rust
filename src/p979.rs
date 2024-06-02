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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.unwrap();
        let mut steps = 0;
        let acc = dfs(&root.borrow(), &mut steps);
        debug_assert_eq!(acc, 0);
        steps
    }
}

fn dfs(node: &TreeNode, steps: &mut i32) -> i32 {
    let TreeNode {
        val,
        ref left,
        ref right,
    } = *node;

    let mut acc = val - 1;

    if let Some(left) = left {
        let lacc = dfs(&left.borrow(), steps);
        acc += lacc;
        *steps += lacc.abs();
    }

    if let Some(right) = right {
        let racc = dfs(&right.borrow(), steps);
        acc += racc;
        *steps += racc.abs();
    }

    acc
}
