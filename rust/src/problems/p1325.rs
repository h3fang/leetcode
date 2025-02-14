pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let mut n = node.borrow_mut();
                let left = Self::remove_leaf_nodes(n.left.take(), target);
                let right = Self::remove_leaf_nodes(n.right.take(), target);
                if left.is_none() && right.is_none() && n.val == target {
                    None
                } else {
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: n.val,
                        left,
                        right,
                    })))
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 2, null, 2, 4]);
        let expected = TreeNode::from_vec(&[1, null, 3, null, 4]);
        assert_eq!(expected, Solution::remove_leaf_nodes(root, 2));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 3, 3, 3, 2]);
        let expected = TreeNode::from_vec(&[1, 3, null, null, 2]);
        assert_eq!(expected, Solution::remove_leaf_nodes(root, 3));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1, 2, null, 2, null, 2]);
        let expected = TreeNode::from_vec(&[1]);
        assert_eq!(expected, Solution::remove_leaf_nodes(root, 2));
    }
}
