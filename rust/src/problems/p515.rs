pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut level = vec![];
        if let Some(n) = root {
            level.push(n);
        }
        while !level.is_empty() {
            let mut max = i32::MIN;
            let mut next = Vec::with_capacity(level.len() * 2);
            for n in level {
                let mut n = n.borrow_mut();
                max = max.max(n.val);
                if let Some(left) = n.left.take() {
                    next.push(left);
                }
                if let Some(right) = n.right.take() {
                    next.push(right);
                }
            }
            level = next;
            result.push(max);
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
        let root = TreeNode::from_vec(&[1, 3, 2, 5, 3, null, 9]);
        assert_eq!(vec![1, 3, 9], Solution::largest_values(root));
    }
}
