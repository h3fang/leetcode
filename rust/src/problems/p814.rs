pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let v = node.borrow().val;
                let l = Self::prune_tree(node.borrow_mut().left.take());
                node.borrow_mut().left = l;
                let r = Self::prune_tree(node.borrow_mut().right.take());
                node.borrow_mut().right = r;
                if v == 0 && node.borrow().left.is_none() && node.borrow().right.is_none() {
                    None
                } else {
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
