pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(p), Some(q)) => {
                let mut p = p.borrow_mut();
                let mut q = q.borrow_mut();
                p.val == q.val
                    && Self::is_same_tree(p.left.take(), q.left.take())
                    && Self::is_same_tree(p.right.take(), q.right.take())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let p = TreeNode::from_vec(&[1, 2, 3]);
        let q = TreeNode::from_vec(&[1, 2, 3]);
        assert!(Solution::is_same_tree(p, q));
    }

    #[test]
    fn case2() {
        let p = TreeNode::from_vec(&[1, 2]);
        let q = TreeNode::from_vec(&[1, null, 2]);
        assert!(!Solution::is_same_tree(p, q));
    }
}
