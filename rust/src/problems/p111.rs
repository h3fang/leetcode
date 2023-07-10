pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut n = node.borrow_mut();
                let left = n.left.take();
                let right = n.right.take();
                1 + match (left, right) {
                    (None, None) => 0,
                    (None, Some(r)) => Self::min_depth(Some(r)),
                    (Some(l), None) => Self::min_depth(Some(l)),
                    (Some(l), Some(r)) => Self::min_depth(Some(l)).min(Self::min_depth(Some(r))),
                }
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        assert_eq!(2, Solution::min_depth(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[2, null, 3, null, 4, null, 5, null, 6]);
        assert_eq!(5, Solution::min_depth(root));
    }
}
