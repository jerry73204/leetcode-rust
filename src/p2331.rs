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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let mut root = root.borrow_mut();

        match root.val {
            0 => false,
            1 => true,
            2 => {
                // OR
                Solution::evaluate_tree(root.left.take())
                    || Solution::evaluate_tree(root.right.take())
            }
            3 => {
                // AND
                Solution::evaluate_tree(root.left.take())
                    && Solution::evaluate_tree(root.right.take())
            }
            _ => unreachable!(),
        }
    }
}
