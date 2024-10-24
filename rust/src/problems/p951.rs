pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(r1), Some(r2)) => {
                let mut r1 = r1.borrow_mut();
                let mut r2 = r2.borrow_mut();
                if r1.val != r2.val {
                    return false;
                }
                (Self::flip_equiv(r1.left.clone(), r2.left.clone())
                    && Self::flip_equiv(r1.right.clone(), r2.right.clone()))
                    || (Self::flip_equiv(r1.left.take(), r2.right.take())
                        && Self::flip_equiv(r2.left.take(), r1.right.take()))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root1 = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6, null, null, null, 7, 8]);
        let root2 = TreeNode::from_vec(&[1, 3, 2, null, 6, 4, 5, null, null, null, null, 8, 7]);
        assert!(Solution::flip_equiv(root1, root2));
    }

    #[test]
    fn case2() {
        let root1 = TreeNode::from_vec(&[]);
        let root2 = TreeNode::from_vec(&[]);
        assert!(Solution::flip_equiv(root1, root2));
    }

    #[test]
    fn case3() {
        let root1 = TreeNode::from_vec(&[]);
        let root2 = TreeNode::from_vec(&[1]);
        assert!(!Solution::flip_equiv(root1, root2));
    }
}
