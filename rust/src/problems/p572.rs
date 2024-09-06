use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn check(
            root: Option<&Rc<RefCell<TreeNode>>>,
            sub_root: Option<&Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (root, sub_root) {
                (Some(a), Some(b)) => {
                    let a = a.borrow();
                    let b = b.borrow();
                    a.val == b.val
                        && check(a.left.as_ref(), b.left.as_ref())
                        && check(a.right.as_ref(), b.right.as_ref())
                }
                (None, None) => true,
                _ => false,
            }
        }

        if check(root.as_ref(), sub_root.as_ref()) {
            return true;
        }

        if let Some(node) = root {
            let n = node.borrow();
            return Self::is_subtree(n.left.clone(), sub_root.clone())
                || Self::is_subtree(n.right.clone(), sub_root);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 4, 5, 1, 2]);
        let sub_root = TreeNode::from_vec(&[4, 1, 2]);
        assert!(Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[3, 4, 5, 1, 2, null, null, null, null, 0]);
        let sub_root = TreeNode::from_vec(&[4, 1, 2]);
        assert!(!Solution::is_subtree(root, sub_root));
    }
}
