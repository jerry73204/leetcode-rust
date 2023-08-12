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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        find(root.as_ref(), (k - 1) as usize).unwrap_err()
    }
}

fn find(node: Option<&Rc<RefCell<TreeNode>>>, nth: usize) -> Result<usize, i32> {
    let node = match node {
        Some(node) => node,
        None => {
            return Ok(0);
        }
    };
    let node = node.borrow();

    let lchild = node.left.as_ref();
    let lsize = find(lchild, nth)?;

    if lsize == nth {
        return Err(node.val);
    }

    let rchild = node.right.as_ref();
    let rsize = find(rchild, nth - lsize - 1)?;

    Ok(lsize + rsize + 1)
}
