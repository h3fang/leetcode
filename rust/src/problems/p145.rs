use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut r = vec![];
        let mut s = vec![root];
        while let Some(node) = s.pop() {
            if let Some(n) = node {
                let n = n.borrow();
                r.push(n.val);
                s.push(n.left.clone());
                s.push(n.right.clone());
            }
        }
        r.reverse();
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, null, 2, 3]);
        assert_eq!(vec![3, 2, 1], Solution::postorder_traversal(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[3, 1, 2]);
        assert_eq!(vec![1, 2, 3], Solution::postorder_traversal(root));
    }
}
