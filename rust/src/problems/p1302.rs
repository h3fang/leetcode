pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        if let Some(r) = root {
            q.push_back(r);
        }
        let mut result = 0;
        while !q.is_empty() {
            let n = q.len();
            let mut sum = 0;
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                sum += node.borrow().val;
                let left = node.borrow_mut().left.take();
                if let Some(left) = left {
                    q.push_back(left);
                }

                let right = node.borrow_mut().right.take();
                if let Some(right) = right {
                    q.push_back(right);
                }
            }
            result = sum;
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
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8]);
        assert_eq!(15, Solution::deepest_leaves_sum(root));
    }
}
