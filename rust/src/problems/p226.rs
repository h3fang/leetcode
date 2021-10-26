use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            {
                let mut n = node.borrow_mut();
                let left = Self::invert_tree(n.right.take());
                let right = Self::invert_tree(n.left.take());
                n.left = left;
                n.right = right;
            }
            node
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]);
        let expected = TreeNode::from_vec(&[4, 7, 2, 9, 6, 3, 1]);
        let result = Solution::invert_tree(root);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[4, 2, 7]);
        let expected = TreeNode::from_vec(&[4, 7, 2]);
        let result = Solution::invert_tree(root);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1]);
        let expected = TreeNode::from_vec(&[1]);
        let result = Solution::invert_tree(root);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }
}
