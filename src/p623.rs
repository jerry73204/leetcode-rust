use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::Solution;

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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }

        let root = match root {
            Some(root) => root,
            None => return None,
        };
        let mut frontier = VecDeque::new();
        frontier.push_back((1, root.clone()));

        while let Some((cur_depth, node)) = frontier.pop_front() {
            if cur_depth == depth - 1 {
                let orig_lchild = node.borrow().left.clone();
                let orig_rchild = node.borrow().right.clone();

                let new_lchild = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: orig_lchild,
                    right: None,
                })));
                let new_rchild = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: None,
                    right: orig_rchild,
                })));

                node.borrow_mut().left = new_lchild;
                node.borrow_mut().right = new_rchild;
            } else {
                if let Some(left) = &node.borrow().left {
                    frontier.push_back((cur_depth + 1, left.clone()));
                }
                if let Some(right) = &node.borrow().right {
                    frontier.push_back((cur_depth + 1, right.clone()));
                }
            }
        }

        Some(root)
    }
}
