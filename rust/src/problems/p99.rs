use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn inorder(
            root: Option<&Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
            first: &mut Option<Rc<RefCell<TreeNode>>>,
            second: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(node) = root {
                let n = node.borrow();
                inorder(n.left.as_ref(), prev, first, second);
                if let Some(p) = prev {
                    if n.val < p.borrow().val {
                        if first.is_none() {
                            *first = Some(p.clone());
                        }
                        if first.is_some() {
                            *second = Some(node.clone());
                        }
                    }
                }
                *prev = Some(node.clone());
                inorder(n.right.as_ref(), prev, first, second);
            }
        }

        let mut prev = None;
        let mut first = None;
        let mut second = None;
        inorder(root.as_ref(), &mut prev, &mut first, &mut second);

        std::mem::swap(
            &mut (first.unwrap().borrow_mut()).val,
            &mut (second.unwrap().borrow_mut()).val,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let mut root = TreeNode::from_vec(&[1, 3, null, null, 2]);
        Solution::recover_tree(&mut root);
        let expected = TreeNode::from_vec(&[3, 1, null, null, 2]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case2() {
        let mut root = TreeNode::from_vec(&[3, 1, 4, null, null, 2]);
        Solution::recover_tree(&mut root);
        let expected = TreeNode::from_vec(&[2, 1, 4, null, null, 3]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case3() {
        let mut root = TreeNode::from_vec(&[2, 3, 1]);
        Solution::recover_tree(&mut root);
        let expected = TreeNode::from_vec(&[2, 1, 3]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }
}
