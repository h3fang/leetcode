use std::{cell::RefCell, rc::Rc};

use crate::utils::tree::TreeNode;

pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

#[allow(clippy::assigning_clones)]
impl BSTIterator {
    pub fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        while let Some(node) = root {
            stack.push(node.clone());
            root = node.borrow().left.clone();
        }
        Self { stack }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        let h = self.stack.pop().unwrap();
        let v = h.borrow().val;
        let mut h = h.borrow().right.clone();
        while let Some(node) = h {
            self.stack.push(node.clone());
            h = node.borrow().left.clone();
        }
        v
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let mut iter = BSTIterator::new(TreeNode::from_vec(&[7, 3, 15, null, null, 9, 20]));
        assert_eq!(3, iter.next());
        assert_eq!(7, iter.next());
        assert!(iter.has_next());
        assert_eq!(9, iter.next());
        assert!(iter.has_next());
        assert_eq!(15, iter.next());
        assert!(iter.has_next());
        assert_eq!(20, iter.next());
        assert!(!iter.has_next());
    }
}
