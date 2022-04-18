use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, pre: *mut *mut TreeNode) {
            if let Some(node) = root {
                let left = node.borrow_mut().left.take();
                inorder(left, pre);
                unsafe {
                    (**pre).right = Some(node.clone());
                    *pre = node.as_ptr();
                }
                let r = node.borrow_mut().right.clone();
                inorder(r, pre);
            }
        }
        let mut dummy = TreeNode::new(0);
        let mut pre: *mut TreeNode = &mut dummy;
        inorder(root, &mut pre);
        dummy.right
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9]);
        let expected = TreeNode::from_vec(&[
            1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9,
        ]);
        assert_eq!(expected, Solution::increasing_bst(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 1, 7]);
        let expected = TreeNode::from_vec(&[1, null, 5, null, 7]);
        assert_eq!(expected, Solution::increasing_bst(root));
    }
}
