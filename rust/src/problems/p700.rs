use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let v = node.borrow().val;
                match v.cmp(&val) {
                    std::cmp::Ordering::Less => {
                        Self::search_bst(node.borrow_mut().right.clone(), val)
                    }
                    std::cmp::Ordering::Equal => Some(node),
                    std::cmp::Ordering::Greater => {
                        Self::search_bst(node.borrow_mut().left.clone(), val)
                    }
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[4, 2, 7, 1, 3]);
        let result = Solution::search_bst(root, 2);
        let expected = TreeNode::from_vec(&[2, 1, 3]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[4, 2, 7, 1, 3]);
        let result = Solution::search_bst(root, 5);
        let expected: Option<Rc<RefCell<TreeNode>>> = None;
        assert_eq!(expected, result);
    }
}
