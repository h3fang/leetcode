use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let mut n1 = node1.borrow_mut();
                let mut n2 = node2.borrow_mut();
                let val = n1.val + n2.val;
                let left = Self::merge_trees(n1.left.take(), n2.left.take());
                let right = Self::merge_trees(n1.right.take(), n2.right.take());
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::null;

    #[test]
    fn case1() {
        let root1 = TreeNode::from_vec(&[1, 3, 2, 5]);
        let root2 = TreeNode::from_vec(&[2, 1, 3, null, 4, null, 7]);
        let result = Solution::merge_trees(root1, root2);
        let expected = TreeNode::from_vec(&[3, 4, 5, 5, 4, null, 7]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }
}
