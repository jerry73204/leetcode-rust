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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        recursion(&preorder, &inorder).unwrap()
    }
}

fn recursion(preorder: &[i32], inorder: &[i32]) -> Result<Option<Rc<RefCell<TreeNode>>>, ()> {
    let (pivot, remain_preorder) = match preorder.split_first() {
        Some(split) => split,
        None => return Ok(None),
    };

    let pivot_indexes = inorder.iter().enumerate().filter(|(_, val)| *val == pivot);

    for (idx, pivot) in pivot_indexes {
        let left_inorder = &inorder[0..idx];
        let right_inorder = &inorder[(idx + 1)..];

        let left_preorder = &remain_preorder[0..left_inorder.len()];
        let right_preorder = &remain_preorder[left_inorder.len()..];

        let lchild = match recursion(left_preorder, left_inorder) {
            Ok(child) => child,
            Err(_) => continue,
        };

        let rchild = match recursion(right_preorder, right_inorder) {
            Ok(child) => child,
            Err(_) => continue,
        };

        let node = TreeNode {
            val: *pivot,
            left: lchild,
            right: rchild,
        };

        return Ok(Some(Rc::new(RefCell::new(node))));
    }

    Err(())
}
