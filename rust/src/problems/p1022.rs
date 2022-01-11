use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32, curr: i32) {
            if let Some(node) = root {
                let n = node.borrow();
                let curr = (curr << 1) + n.val;
                match (&n.left, &n.right) {
                    (None, None) => *sum += curr,
                    (None, Some(r)) => dfs(Some(r), sum, curr),
                    (Some(l), None) => dfs(Some(l), sum, curr),
                    (Some(l), Some(r)) => {
                        dfs(Some(l), sum, curr);
                        dfs(Some(r), sum, curr);
                    }
                }
            }
        }
        let mut sum = 0;
        dfs(root.as_ref(), &mut sum, 0);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(22, Solution::sum_root_to_leaf(root));
    }
}
