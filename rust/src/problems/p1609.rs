use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        let mut q = VecDeque::from([root.unwrap()]);
        let mut odd = false;
        while !q.is_empty() {
            let n = q.len();
            let mut prev = 0;
            for i in 0..n {
                let node = q.pop_front().unwrap();
                let mut curr = node.borrow_mut();
                if curr.val % 2 != i32::from(!odd) {
                    return false;
                }
                if i > 0 {
                    if odd {
                        if curr.val >= prev {
                            return false;
                        }
                    } else if curr.val <= prev {
                        return false;
                    }
                }
                prev = curr.val;
                if let Some(left) = curr.left.take() {
                    q.push_back(left);
                }
                if let Some(right) = curr.right.take() {
                    q.push_back(right);
                }
            }
            odd = !odd;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 9, 1, 3, 5, 7]);
        assert!(!Solution::is_even_odd_tree(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[11, 8, 6, 1, 3, 9, 11, 30, 20, 18, 16, 12, 10, 4, 2, 17]);
        assert!(Solution::is_even_odd_tree(root));
    }
}
