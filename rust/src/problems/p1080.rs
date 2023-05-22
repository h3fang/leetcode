pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                limit -= node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return if limit > 0 { None } else { Some(node) };
                }
                let l = Self::sufficient_subset(node.borrow_mut().left.take(), limit);
                node.borrow_mut().left = l;
                let r = Self::sufficient_subset(node.borrow_mut().right.take(), limit);
                node.borrow_mut().right = r;
                if node.borrow().left.is_some() || node.borrow().right.is_some() {
                    Some(node)
                } else {
                    None
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
        let root = TreeNode::from_vec(&[1, 2, 3, 4, -99, -99, 7, 8, 9, -99, -99, 12, 13, -99, 14]);
        let expected = TreeNode::from_vec(&[1, 2, 3, 4, null, null, 7, 8, 9, null, 14]);
        assert_eq!(expected, Solution::sufficient_subset(root, 1));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 4, 8, 11, null, 17, 4, 7, 1, null, null, 5, 3]);
        let expected = TreeNode::from_vec(&[5, 4, 8, 11, null, 17, 4, 7, null, null, null, 5]);
        assert_eq!(expected, Solution::sufficient_subset(root, 22));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1, 2, -3, -5, null, 4, null]);
        let expected = TreeNode::from_vec(&[1, null, -3, 4]);
        assert_eq!(expected, Solution::sufficient_subset(root, -1));
    }
}
