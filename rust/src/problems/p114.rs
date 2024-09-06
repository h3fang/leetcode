use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

#[allow(clippy::assigning_clones)]
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        match root {
            None => {}
            Some(node) => {
                let mut n = node.borrow_mut();

                let mut left = n.left.take();
                Self::flatten(&mut left);
                let mut right = n.right.take();
                Self::flatten(&mut right);

                n.right = left;
                let mut h = n.right.clone();
                drop(n);

                let mut prev = node.clone();
                while let Some(node) = h {
                    let n = node.borrow_mut();
                    h = n.right.clone();
                    prev = node.clone();
                }
                prev.borrow_mut().right = right;
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
        let mut root = TreeNode::from_vec(&[1, 2, 5, 3, 4, null, 6]);
        let expected = TreeNode::from_vec(&[1, null, 2, null, 3, null, 4, null, 5, null, 6]);
        Solution::flatten(&mut root);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case2() {
        let mut root = TreeNode::from_vec(&[0]);
        let expected = TreeNode::from_vec(&[0]);
        Solution::flatten(&mut root);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }
}
