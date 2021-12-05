use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                Some(node) => {
                    let mut n = node.borrow_mut();
                    let left = dfs(n.left.take());
                    let right = dfs(n.right.take());
                    (
                        n.val + left.1 + right.1,
                        left.0.max(left.1) + right.0.max(right.1),
                    )
                }
                None => (0, 0),
            }
        }
        let r = dfs(root);
        r.0.max(r.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 2, 3, null, 3, null, 1]);
        assert_eq!(7, Solution::rob(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[3, 4, 5, 1, 3, null, 1]);
        assert_eq!(9, Solution::rob(root));
    }
}
