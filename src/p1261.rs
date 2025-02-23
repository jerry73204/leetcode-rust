//! # 1261. Find Elements in a Contaminated Binary Tree
//! [Submission](https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/submissions/1553010989)

use std::{cell::RefCell, rc::Rc};

#[allow(unused)]
struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl FindElements {
    #[allow(unused)]
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::with_capacity(20);

        if let Some(root) = &root {
            stack.push((1, root.clone()));
        }

        while let Some((value, curr)) = stack.pop() {
            curr.borrow_mut().val = value;

            let borrow = curr.borrow();
            if let Some(left) = &borrow.left {
                stack.push((value * 2, left.clone()));
            }

            if let Some(right) = &borrow.right {
                stack.push((value * 2 + 1, right.clone()));
            }
        }

        Self { root }
    }

    #[allow(unused)]
    fn find(&self, target: i32) -> bool {
        let Some(mut curr) = self.root.clone() else {
            return false;
        };

        let bits = target + 1;
        let mut mask = 1 << (i32::BITS - bits.leading_zeros() - 1);
        mask >>= 1;

        while mask != 0 {
            let is_right = (bits & mask) != 0;
            mask >>= 1;

            if is_right {
                let Some(right) = curr.borrow().right.clone() else {
                    return false;
                };
                curr = right.clone();
            } else {
                let Some(left) = curr.borrow().left.clone() else {
                    return false;
                };
                curr = left.clone();
            }
        }

        true
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

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
