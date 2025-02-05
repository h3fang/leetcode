pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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

        result.reverse();
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
        let expected = vec![vec![15, 7], vec![9, 20], vec![3]];
        assert_eq!(expected, Solution::level_order_bottom(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1]);
        let expected = vec![vec![1]];
        assert_eq!(expected, Solution::level_order_bottom(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[]);
        assert!(Solution::level_order_bottom(root).is_empty());
    }
}
