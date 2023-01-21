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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output = vec![];
        solve_recursive(root, &mut output);
        output
    }
}

fn solve_recursive(root: Option<Rc<RefCell<TreeNode>>>, output: &mut Vec<i32>) {
    let root = match root {
        Some(root) => root,
        None => return,
    };

    let root = root.borrow();

    output.push(root.val);

    solve_recursive(root.left.clone(), output);
    solve_recursive(root.right.clone(), output);
}
