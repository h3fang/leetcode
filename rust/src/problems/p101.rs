use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut level = vec![root];
        while !level.is_empty() {
            let mut next = vec![];
            let m = level.len();
            for i in 0..m {
                if let Some(n) = &level[i] {
                    let mut n = n.borrow_mut();
                    if let Some(right) = &level[m - 1 - i] {
                        if i < m / 2 && right.borrow().val != n.val {
                            return false;
                        }
                    } else {
                        return false;
                    }
                    next.push(n.left.take());
                    next.push(n.right.take());
                }
            }
            level = next;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 2, 3, 4, 4, 3]);
        assert!(Solution::is_symmetric(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 2, null, 3, null, 3]);
        assert!(!Solution::is_symmetric(root));
    }
}
