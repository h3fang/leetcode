pub struct Solution;

use crate::utils::tree::TreeNode;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub struct FindElements {
    vals: HashSet<i32>,
}

impl FindElements {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn dfs(r: Rc<RefCell<TreeNode>>, x: i32, vals: &mut HashSet<i32>) {
            vals.insert(x);
            if let Some(r) = r.borrow_mut().left.take() {
                dfs(r, 2 * x + 1, vals);
            }
            if let Some(r) = r.borrow_mut().right.take() {
                dfs(r, 2 * x + 2, vals);
            }
        }
        let mut vals = HashSet::new();
        let r = root.unwrap();
        dfs(r, 0, &mut vals);
        Self { vals }
    }

    pub fn find(&self, target: i32) -> bool {
        self.vals.contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[-1, null, -1]);
        let fe = FindElements::new(root);
        assert!(!fe.find(1));
        assert!(fe.find(2));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[-1, -1, -1, -1, -1]);
        let fe = FindElements::new(root);
        assert!(fe.find(1));
        assert!(fe.find(3));
        assert!(!fe.find(5));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[-1, null, -1, -1, null, -1]);
        let fe = FindElements::new(root);
        assert!(fe.find(2));
        assert!(!fe.find(3));
        assert!(!fe.find(4));
        assert!(fe.find(5));
    }
}
