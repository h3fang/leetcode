pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        q.push_back(root.unwrap());
        let mut result = 0;
        while !q.is_empty() {
            let n = q.len();
            let mut vals = Vec::with_capacity(n);
            for i in 0..n {
                let node = q.pop_front().unwrap();
                let mut node = node.borrow_mut();
                let v = (node.val as i64) << 32;
                vals.push(v + i as i64);
                if let Some(l) = node.left.take() {
                    q.push_back(l);
                }
                if let Some(r) = node.right.take() {
                    q.push_back(r);
                }
            }
            vals.sort_unstable();
            let mut i = 0;
            while i < n {
                let j = (vals[i] & 0xFFFFFFFF) as usize;
                if j != i {
                    vals.swap(i, j);
                    result += 1;
                } else {
                    i += 1;
                }
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
        let root = TreeNode::from_vec(&[1, 4, 3, 7, 6, 8, 5, null, null, null, null, 9, null, 10]);
        assert_eq!(3, Solution::minimum_operations(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 3, 2, 7, 6, 5, 4]);
        assert_eq!(3, Solution::minimum_operations(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(0, Solution::minimum_operations(root));
    }
}
