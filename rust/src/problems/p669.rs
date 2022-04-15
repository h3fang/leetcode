pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let v = node.borrow().val;
                if v < low {
                    Self::trim_bst(node.borrow_mut().right.take(), low, high)
                } else if v > high {
                    Self::trim_bst(node.borrow_mut().left.take(), low, high)
                } else {
                    let left = node.borrow_mut().left.take();
                    node.borrow_mut().left = Self::trim_bst(left, low, high);
                    let right = node.borrow_mut().right.take();
                    node.borrow_mut().right = Self::trim_bst(right, low, high);
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
        let root = TreeNode::from_vec(&[1, 0, 2]);
        let low = 1;
        let high = 2;
        let expected = TreeNode::from_vec(&[1, null, 2]);
        assert_eq!(expected, Solution::trim_bst(root, low, high));
    }
}
