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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.unwrap();

        let mut stack = vec![(root, false)];
        let mut prev = None;
        let mut min_diff = i32::MAX;

        while let Some((node, is_visiting)) = stack.pop() {
            if is_visiting {
                let node_ref = node.borrow();

                if let Some(prev) = prev {
                    min_diff = cmp::min(node_ref.val - prev, min_diff);
                }

                prev = Some(node_ref.val);
            } else {
                let node_ref = node.borrow();

                if let Some(rchild) = &node_ref.right {
                    stack.push((rchild.clone(), false));
                }

                stack.push((node.clone(), true));

                if let Some(lchild) = &node_ref.left {
                    stack.push((lchild.clone(), false));
                }
            }
        }

        min_diff
    }
}
