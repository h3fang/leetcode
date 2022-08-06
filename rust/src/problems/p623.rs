pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        mut depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }
        let mut level = vec![root.as_ref().unwrap().clone()];
        depth -= 1;
        while depth > 1 {
            let mut next = vec![];
            for n in level {
                if let Some(left) = &n.borrow_mut().left {
                    next.push(left.clone());
                }
                if let Some(right) = &n.borrow_mut().right {
                    next.push(right.clone());
                }
            }
            level = next;
            depth -= 1;
        }
        for n in level {
            let left = n.borrow_mut().left.take();
            let right = n.borrow_mut().right.take();
            n.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left,
                right: None,
            })));
            n.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right,
            })));
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[4, 2, 6, 3, 1, 5]);
        let val = 1;
        let depth = 2;
        let expected = TreeNode::from_vec(&[4, 1, 1, 2, null, null, 6, 3, 1, 5]);
        assert_eq!(expected, Solution::add_one_row(root, val, depth));
    }
}
