pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut level = vec![];
        if let Some(r) = root {
            level.push(r);
        }
        while !level.is_empty() {
            let mut row = Vec::with_capacity(level.len());
            let mut next = Vec::with_capacity(level.len() * 2);
            for node in level {
                let mut n = node.borrow_mut();
                row.push(n.val);
                if let Some(node) = n.left.take() {
                    next.push(node);
                }
                if let Some(node) = n.right.take() {
                    next.push(node);
                }
            }
            result.push(row);
            level = next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(expected, Solution::level_order(root));
    }
}
