pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let mut n = node.borrow_mut();
                n.left = Self::prune_tree(n.left.take());
                n.right = Self::prune_tree(n.right.take());
                if n.val == 0 && n.left.is_none() && n.right.is_none() {
                    None
                } else {
                    drop(n);
                    Some(node)
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
        let root = TreeNode::from_vec(&[1, null, 0, 0, 1]);
        let expected = TreeNode::from_vec(&[1, null, 0, null, 1]);
        assert_eq!(expected, Solution::prune_tree(root));
    }
}
