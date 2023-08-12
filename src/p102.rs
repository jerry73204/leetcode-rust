use crate::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = match root {
            Some(root) => root,
            None => return vec![],
        };
        let mut queue = VecDeque::new();
        queue.push_back((root, false));

        let mut curr_color = false;
        let mut output = vec![vec![]];

        while let Some((node, color)) = queue.pop_front() {
            if color != curr_color {
                output.push(vec![]);
                curr_color = color;
            }
            let node = node.borrow();

            output.last_mut().unwrap().push(node.val);

            if let Some(lchild) = node.left.as_ref() {
                queue.push_back((lchild.clone(), !color));
            }

            if let Some(rchild) = node.right.as_ref() {
                queue.push_back((rchild.clone(), !color));
            }
        }

        output
    }
}
