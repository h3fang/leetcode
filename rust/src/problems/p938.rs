use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(node) => {
                let mut n = node.borrow_mut();
                if n.val > high {
                    Self::range_sum_bst(n.left.take(), low, high)
                } else if n.val < low {
                    Self::range_sum_bst(n.right.take(), low, high)
                } else {
                    n.val
                        + Self::range_sum_bst(n.left.take(), low, high)
                        + Self::range_sum_bst(n.right.take(), low, high)
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
        let root = TreeNode::from_vec(&[10, 5, 15, 3, 7, null, 18]);
        let low = 7;
        let high = 15;
        assert_eq!(32, Solution::range_sum_bst(root, low, high));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[10, 5, 15, 3, 7, 13, 18, 1, null, 6]);
        let low = 6;
        let high = 10;
        assert_eq!(23, Solution::range_sum_bst(root, low, high));
    }
}
