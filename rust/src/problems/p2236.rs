pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let n = root.as_ref().unwrap().borrow();
        let sum = n.left.as_ref().unwrap().borrow().val + n.right.as_ref().unwrap().borrow().val;
        n.val == sum
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::TreeNode;

    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_tree(TreeNode::from_vec(&[10, 4, 6])));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_tree(TreeNode::from_vec(&[5, 3, 1])));
    }
}
