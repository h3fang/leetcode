pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mask: i32, result: &mut i32) {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                let m = mask ^ (1 << n.val);
                let l = n.left.take();
                let r = n.right.take();
                if l.is_none() && r.is_none() {
                    if m == 0 || (m & (m - 1)) == 0 {
                        *result += 1;
                    }
                } else {
                    dfs(l, m, result);
                    dfs(r, m, result);
                }
            }
        }
        let mut result = 0;
        dfs(root, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[2, 3, 1, 3, 1, null, 1]);
        assert_eq!(2, Solution::pseudo_palindromic_paths(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[2, 1, 1, 1, 3, null, null, null, null, null, 1]);
        assert_eq!(1, Solution::pseudo_palindromic_paths(root));
    }
}
