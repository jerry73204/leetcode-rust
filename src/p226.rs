use crate::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem;
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let mut queue: VecDeque<_> = Some(root.clone()).into_iter().collect();

        while let Some(node) = queue.pop_front() {
            let mut node = node.borrow_mut();
            let TreeNode { left, right, .. } = &mut *node;
            mem::swap(left, right);

            let child_iter = right.clone().into_iter().chain(left.clone());
            queue.extend(child_iter);
        }

        Some(root)
    }
}
