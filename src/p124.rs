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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.as_ref().unwrap();
        let mut max_path_sum = root.borrow().val;
        dfs(root, &mut max_path_sum);
        max_path_sum
    }
}

fn dfs(node: &Rc<RefCell<TreeNode>>, max_path_sum: &mut i32) -> i32 {
    let node = node.borrow();

    match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => {
            *max_path_sum = cmp::max(*max_path_sum, node.val);
            node.val
        }
        (Some(lchild), None) => {
            let left_trail_sum = dfs(lchild, max_path_sum);
            let max_trail_sum = cmp::max(left_trail_sum, 0) + node.val;
            *max_path_sum = cmp::max(*max_path_sum, max_trail_sum);
            max_trail_sum
        }
        (None, Some(rchild)) => {
            let right_trail_sum = dfs(rchild, max_path_sum);
            let max_trail_sum = cmp::max(right_trail_sum, 0) + node.val;
            *max_path_sum = cmp::max(*max_path_sum, max_trail_sum);
            max_trail_sum
        }
        (Some(lchild), Some(rchild)) => {
            let left_trail_sum = dfs(lchild, max_path_sum);
            let right_trail_sum = dfs(rchild, max_path_sum);
            let max_trail_sum = left_trail_sum.max(right_trail_sum).max(0) + node.val;
            *max_path_sum = cmp::max(
                *max_path_sum,
                cmp::max(left_trail_sum, 0) + cmp::max(right_trail_sum, 0) + node.val,
            );
            max_trail_sum
        }
    }
}
