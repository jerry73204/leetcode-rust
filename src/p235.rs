use crate::Solution;
use std::cell::RefCell;
use std::ops::{Bound, RangeBounds};
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
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_node = p.unwrap();
        let p_node = p_node.borrow();
        let p_val = p_node.val;

        let q_node = q.unwrap();
        let q_node = q_node.borrow();
        let q_val = q_node.val;

        let mut bound = (Bound::Unbounded, Bound::Unbounded);

        loop {
            let node = root.unwrap();
            let node_ref = node.borrow();
            let val = node_ref.val;

            if [p_val, q_val].contains(&val) {
                return Some(node.clone());
            }

            let lbound = (bound.0, Bound::Excluded(val));
            let has_p = lbound.contains(&p_val);
            let has_q = lbound.contains(&q_val);

            match (has_p, has_q) {
                (true, true) => {
                    root = node_ref.left.clone();
                    bound = lbound;
                    continue;
                }
                (false, false) => {
                    root = node_ref.right.clone();
                    bound = (Bound::Excluded(val), bound.1);
                    continue;
                }
                _ => return Some(node.clone()),
            }
        }
    }
}
