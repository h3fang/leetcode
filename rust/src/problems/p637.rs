pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }
        let mut q = VecDeque::new();
        q.push_back(root.unwrap());
        while !q.is_empty() {
            let n = q.len();
            let mut sum = 0.0;
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                let mut n = node.borrow_mut();
                sum += n.val as f64;
                if let Some(left) = n.left.take() {
                    q.push_back(left);
                }
                if let Some(right) = n.right.take() {
                    q.push_back(right);
                }
            }
            result.push(sum / n as f64);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    fn assert_close(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(b) {
            assert!((x - y).abs() < 1e-5);
        }
    }

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        let result = Solution::average_of_levels(root);
        assert_close(&[3.00000, 14.50000, 11.00000], &result);
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[3, 9, 20, 15, 7]);
        let result = Solution::average_of_levels(root);
        assert_close(&[3.00000, 14.50000, 11.00000], &result);
    }
}
