use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_match(a: Option<&Rc<RefCell<TreeNode>>>, b: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match b {
                Some(b) => {
                    if let Some(a) = a {
                        let a = a.borrow();
                        let b = b.borrow();
                        a.val == b.val
                            && is_match(a.left.as_ref(), b.left.as_ref())
                            && is_match(a.right.as_ref(), b.right.as_ref())
                    } else {
                        false
                    }
                }
                None => true,
            }
        }

        if b.is_none() {
            return false;
        }
        is_match(a.as_ref(), b.as_ref()) || {
            if let Some(a) = a {
                let a = a.borrow();
                Self::is_sub_structure(a.left.clone(), b.clone())
                    || Self::is_sub_structure(a.right.clone(), b)
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let a = TreeNode::from_vec(&[1, 2, 3]);
        let b = TreeNode::from_vec(&[3, 1]);
        assert!(!Solution::is_sub_structure(a, b));
    }

    #[test]
    fn case2() {
        let a = TreeNode::from_vec(&[3, 4, 5, 1, 2]);
        let b = TreeNode::from_vec(&[4, 1]);
        assert!(Solution::is_sub_structure(a, b));
    }
}
