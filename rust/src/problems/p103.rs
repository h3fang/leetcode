pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut left_to_right = true;
        let mut q = vec![root];
        let mut result = vec![];
        while !q.is_empty() {
            let mut next = vec![];
            let mut row = vec![];
            for n in q.into_iter().flatten() {
                let mut n = n.borrow_mut();
                row.push(n.val);
                if left_to_right {
                    next.push(n.left.take());
                    next.push(n.right.take());
                } else {
                    next.push(n.right.take());
                    next.push(n.left.take());
                }
            }
            if !row.is_empty() {
                result.push(row);
            }
            next.reverse();
            left_to_right = !left_to_right;
            q = next;
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
        let expected = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(expected, Solution::zigzag_level_order(root));
    }
}
