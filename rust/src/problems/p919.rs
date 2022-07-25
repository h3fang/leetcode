use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::tree::TreeNode;

pub struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    q: VecDeque<Rc<RefCell<TreeNode>>>,
}

impl CBTInserter {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut q = VecDeque::new();
        q.push_back(root.as_ref().unwrap().clone());
        while !q.is_empty() {
            let n = q.front().unwrap().clone();
            if let Some(l) = &n.borrow().left {
                q.push_back(l.clone());
            }
            if let Some(r) = &n.borrow().right {
                q.push_back(r.clone());
                q.pop_front();
            } else {
                break;
            };
        }
        Self { root, q }
    }

    pub fn insert(&mut self, val: i32) -> i32 {
        let parent = self.q.front().unwrap().clone();
        let r = parent.borrow().val;
        let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        self.q.push_back(node.as_ref().unwrap().clone());
        if parent.borrow().left.is_none() {
            parent.borrow_mut().left = node;
        } else if parent.borrow().right.is_none() {
            parent.borrow_mut().right = node;
            self.q.pop_front();
        }
        r
    }

    pub fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2]);
        let mut ci = CBTInserter::new(root);
        assert_eq!(1, ci.insert(3));
        assert_eq!(2, ci.insert(4));
        let root = TreeNode::from_vec(&[1, 2, 3, 4]);
        assert_eq!(root, ci.get_root());
    }
}
