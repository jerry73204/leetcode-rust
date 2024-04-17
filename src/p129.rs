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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        let root = match root {
            Some(root) => root,
            None => return 0,
        };
        let mut frontiers = vec![(root.borrow().val, root.clone())];

        while let Some((prefix_sum, curr)) = frontiers.pop() {
            let borrow = curr.borrow();

            if let Some(left) = &borrow.left {
                frontiers.push((prefix_sum * 10 + left.borrow().val, left.clone()));
            }

            if let Some(right) = &borrow.right {
                frontiers.push((prefix_sum * 10 + right.borrow().val, right.clone()));
            }

            if borrow.left.is_none() && borrow.right.is_none() {
                sum += prefix_sum;
            }
        }

        sum
    }
}
