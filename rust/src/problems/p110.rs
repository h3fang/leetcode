use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(n) => {
                    let n = n.borrow();

                    let left = dfs(n.left.as_ref());
                    if left < 0 {
                        return left;
                    }

                    let right = dfs(n.right.as_ref());
                    if right < 0 {
                        return right;
                    }

                    if (left - right).abs() < 2 {
                        1 + left.max(right)
                    } else {
                        -1
                    }
                }
                None => 0,
            }
        }
        dfs(root.as_ref()) >= 0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 2, 3, 3, null, null, 4, 4]);
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[]);
        assert!(Solution::is_balanced(root));
    }
}
