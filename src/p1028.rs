//! # 1028. Recover a Tree From Preorder Traversal
//! [Submission](https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/submissions/1552983510)

use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut remain = traversal.as_bytes();
        let iter = std::iter::from_fn(|| {
            let mut iter = remain.iter().copied().enumerate();
            let (depth, _) = (&mut iter).find(|(_, b)| *b != b'-')?;
            let end = match iter.find(|(_, b)| *b == b'-') {
                Some((end, _)) => end,
                None => remain.len(),
            };
            let value = remain[depth..end]
                .iter()
                .fold(0i32, |val, &b| val * 10 + (b - b'0') as i32);
            remain = &remain[end..];
            Some((depth, value))
        });

        let mut stack: Vec<TreeNode> = Vec::with_capacity(1000);

        for (depth, value) in iter {
            while stack.len() > depth {
                let child = stack.pop().unwrap();
                let parent = stack.last_mut().unwrap();
                insert_child(parent, child);
            }
            stack.push(TreeNode::new(value));
        }

        let root = loop {
            let node = stack.pop().unwrap();
            let Some(parent) = stack.last_mut() else {
                break node;
            };
            insert_child(parent, node);
        };

        wrap_node(root)
    }
}

fn insert_child(parent: &mut TreeNode, child: TreeNode) {
    if parent.left.is_none() {
        parent.left = wrap_node(child);
    } else {
        debug_assert!(parent.right.is_none());
        parent.right = wrap_node(child);
    }
}

fn wrap_node(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(node)))
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
