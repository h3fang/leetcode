pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        q.push_back(root.unwrap());
        let mut level = 1;
        let mut max = i32::MIN;
        let mut result = 0;
        while !q.is_empty() {
            let n = q.len();
            let mut sum = 0;
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                let mut node = node.borrow_mut();
                sum += node.val;
                if let Some(left) = node.left.take() {
                    q.push_back(left);
                }
                if let Some(right) = node.right.take() {
                    q.push_back(right);
                }
            }
            if sum > max {
                max = sum;
                result = level;
            }
            level += 1;
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
        let root = TreeNode::from_vec(&[1, 7, 0, 7, -8, null, null]);
        assert_eq!(2, Solution::max_level_sum(root));
    }
}
