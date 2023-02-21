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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = match root {
            Some(root) => root,
            None => return vec![],
        };

        let root = root.borrow();
        let mut output = { vec![vec![root.val]] };
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, bool)> = root
            .left
            .clone()
            .into_iter()
            .chain(root.right.clone())
            .map(|node| (node, true))
            .collect();
        let mut prev_r2l = false;

        drop(root);

        while let Some((node, curr_r2l)) = queue.pop_front() {
            if prev_r2l != curr_r2l {
                output.push(vec![]);
            }
            prev_r2l = curr_r2l;

            let node = node.borrow();
            output.last_mut().unwrap().push(node.val);

            let child_iter = node
                .left
                .clone()
                .into_iter()
                .chain(node.right.clone())
                .map(|node| (node, !curr_r2l));
            queue.extend(child_iter);
        }

        output.iter_mut().skip(1).step_by(2).for_each(|vec| {
            vec.reverse();
        });

        output
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use crate::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn p103_test() {
        macro_rules! make_node {
            ($val:expr, $left:expr, $right:expr) => {
                Rc::new(RefCell::new(TreeNode {
                    val: $val,
                    left: $left,
                    right: $right,
                }))
            };
        }

        let root = Some(make_node!(
            3,
            Some(make_node!(9, None, None)),
            Some(make_node!(
                20,
                Some(make_node!(15, None, None)),
                Some(make_node!(7, None, None))
            ))
        ));

        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}
