use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut q = VecDeque::from(vec![root]);
        while let Some(n) = q.pop_front() {
            if let Some(n) = n {
                let n = n.borrow();
                result.push(n.val);
                q.push_back(n.left.clone());
                q.push_back(n.right.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        assert_eq!(vec![3, 9, 20, 15, 7], Solution::level_order(root));
    }
}
