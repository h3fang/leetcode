pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if node.borrow().val > val {
                    if node.borrow().right.is_none() {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    } else {
                        let right = node.borrow_mut().right.take();
                        node.borrow_mut().right = Self::insert_into_max_tree(right, val);
                    }
                    Some(node)
                } else {
                    Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: Some(node),
                        right: None,
                    })))
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
        let root = TreeNode::from_vec(&[4, 1, 3, null, null, 2]);
        let expected = TreeNode::from_vec(&[5, 4, null, 1, 3, null, null, 2]);
        assert_eq!(expected, Solution::insert_into_max_tree(root, 5));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 2, 4, null, 1]);
        let expected = TreeNode::from_vec(&[5, 2, 4, null, 1, null, 3]);
        assert_eq!(expected, Solution::insert_into_max_tree(root, 3));
    }
}
