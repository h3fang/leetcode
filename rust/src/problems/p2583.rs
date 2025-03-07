pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut level = if let Some(root) = root {
            vec![root]
        } else {
            return -1;
        };
        let k = k as usize;
        let mut q = BinaryHeap::new();
        while !level.is_empty() {
            let mut next = Vec::with_capacity(level.len() * 2);
            let mut sum = 0;
            for n in level {
                let mut n = n.borrow_mut();
                sum += n.val as i64;
                if let Some(e) = n.left.take() {
                    next.push(e);
                }
                if let Some(e) = n.right.take() {
                    next.push(e);
                }
            }
            q.push(-sum);
            if q.len() > k {
                q.pop();
            }
            level = next;
        }
        if q.len() == k {
            -q.pop().unwrap()
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 8, 9, 2, 1, 3, 7, 4, 6]);
        assert_eq!(13, Solution::kth_largest_level_sum(root, 2));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, null, 3]);
        assert_eq!(3, Solution::kth_largest_level_sum(root, 1));
    }
}
