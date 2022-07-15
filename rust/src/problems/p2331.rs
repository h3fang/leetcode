pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.unwrap();
        let mut n = node.borrow_mut();
        if n.left.is_none() {
            return n.val == 1;
        }
        let l = Self::evaluate_tree(n.left.take());
        let r = Self::evaluate_tree(n.right.take());
        if n.val == 2 {
            l || r
        } else {
            l && r
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[2, 1, 3, null, null, 0, 1]);
        assert_eq!(true, Solution::evaluate_tree(root));
    }
}
