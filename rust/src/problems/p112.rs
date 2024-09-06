pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            let t = target_sum - n.val;
            if t == 0 && n.left.is_none() && n.right.is_none() {
                return true;
            }
            Self::has_path_sum(n.left.take(), t) || Self::has_path_sum(n.right.take(), t)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1]);
        assert!(Solution::has_path_sum(root, 22));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 3]);
        assert!(!Solution::has_path_sum(root, 5));
    }
}
