//! # 889. Construct Binary Tree from Preorder and Postorder
//! [Submission](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/submissions/1552791653)

use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        traverse(&preorder, &postorder)
    }
}

pub fn traverse(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let (&value, preorder) = preorder.split_first()?;
    let (&value2, postorder) = postorder.split_last()?;
    debug_assert_eq!(value, value2);

    let Some(&next_value) = preorder.first() else {
        debug_assert!(postorder.is_empty());
        return Some(Rc::new(RefCell::new(TreeNode {
            val: value,
            left: None,
            right: None,
        })));
    };

    let (left_postorder, right_postorder) = {
        let (idx, _) = postorder
            .iter()
            .enumerate()
            .find(|(_, &val)| val == next_value)
            .unwrap();
        postorder.split_at(idx + 1)
    };
    let (left_preorder, right_preorder) = preorder.split_at(left_postorder.len());

    let left_node = traverse(left_preorder, left_postorder);
    let right_node = traverse(right_preorder, right_postorder);

    Some(Rc::new(RefCell::new(TreeNode {
        val: value,
        left: left_node,
        right: right_node,
    })))
}

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
