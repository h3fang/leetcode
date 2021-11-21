use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, k: i32, result: &mut i32) -> i32 {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let right = dfs(n.right.as_ref(), k, result);
                    if right == k {
                        k
                    } else if right + 1 == k {
                        *result = n.val;
                        k
                    } else {
                        let left = dfs(n.left.as_ref(), k - right - 1, result);
                        right + 1 + left
                    }
                }
                None => 0,
            }
        }
        let mut result = 0;
        dfs(root.as_ref(), k, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 1, 4, null, 2]);
        assert_eq!(4, Solution::kth_largest(root, 1));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 3, 6, 2, 4, null, null, 1]);
        assert_eq!(4, Solution::kth_largest(root, 3));
    }
}
