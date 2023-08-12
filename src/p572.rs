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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let root = root.unwrap();
        let sub_root = sub_root.unwrap();
        dfs(&root, &sub_root)
    }
}

fn dfs(root: &Rc<RefCell<TreeNode>>, sub_root: &Rc<RefCell<TreeNode>>) -> bool {
    let root = root.borrow();

    if let Some(lchild) = &root.left {
        if dfs(lchild, sub_root) {
            return true;
        }
    }

    if let Some(rchild) = &root.right {
        if dfs(rchild, sub_root) {
            return true;
        }
    }

    if *root == *sub_root.borrow() {
        return true;
    }

    false
}
