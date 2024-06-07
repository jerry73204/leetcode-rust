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
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut max_dist = 0;
        find_start(&root.unwrap().borrow(), start, &mut max_dist);
        max_dist
    }
}

fn find_start(node: &TreeNode, start: i32, max_dist: &mut i32) -> (bool, i32) {
    if start == node.val {
        *max_dist = height_of(node);
        (true, 0)
    } else {
        let mut dist_to_start = None;

        let left_dist = if let Some(child) = &node.left {
            let (found, dist) = find_start(&child.borrow(), start, max_dist);
            if found {
                dist_to_start = Some(dist + 1);
            }
            dist + 1
        } else {
            0
        };

        let right_dist = if let Some(child) = &node.right {
            let (found, dist) = find_start(&child.borrow(), start, max_dist);
            if found {
                dist_to_start = Some(dist + 1);
            }
            dist + 1
        } else {
            0
        };

        if let Some(dist_to_start) = dist_to_start {
            *max_dist = (*max_dist).max(left_dist + right_dist);
            (true, dist_to_start)
        } else {
            (false, left_dist.max(right_dist))
        }
    }
}

fn height_of(node: &TreeNode) -> i32 {
    let mut height = 0;

    if let Some(left) = &node.left {
        height = height.max(height_of(&left.borrow()) + 1);
    }

    if let Some(right) = &node.right {
        height = height.max(height_of(&right.borrow()) + 1);
    }

    height
}
