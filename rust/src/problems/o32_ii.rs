use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut q = VecDeque::from(vec![root]);
        while !q.is_empty() {
            let m = q.len();
            let mut row = Vec::with_capacity(m);
            for _i in 0..m {
                if let Some(n) = q.pop_front().unwrap() {
                    let n = n.borrow();
                    row.push(n.val);
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                }
            }
            if !row.is_empty() {
                result.push(row);
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
        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(root)
        );
    }
}
