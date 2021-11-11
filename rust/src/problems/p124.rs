use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let left = dfs(n.left.as_ref(), result);
                    let right = dfs(n.right.as_ref(), result);
                    *result = (*result).max(left + right + n.val);
                    (n.val + right.max(left)).max(0)
                }
                None => 0,
            }
        }
        let mut result = i32::MIN;
        dfs(root.as_ref(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3]);
        assert_eq!(6, Solution::max_path_sum(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[-10, 9, 20, null, null, 15, 7]);
        assert_eq!(42, Solution::max_path_sum(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[-3]);
        assert_eq!(-3, Solution::max_path_sum(root));
    }

    #[test]
    fn case4() {
        let root = TreeNode::from_vec(&[2, -1]);
        assert_eq!(2, Solution::max_path_sum(root));
    }

    #[test]
    fn case5() {
        let root = TreeNode::from_vec(&[-1, 5, null, 4, null, null, 2, -4]);
        assert_eq!(11, Solution::max_path_sum(root));
    }
}
