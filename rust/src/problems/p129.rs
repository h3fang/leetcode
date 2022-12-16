use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, curr: i32) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let n = node.borrow();
                    let curr = curr * 10 + n.val;
                    if n.left.is_none() && n.right.is_none() {
                        curr
                    } else {
                        dfs(n.right.as_ref(), curr) + dfs(n.left.as_ref(), curr)
                    }
                }
            }
        }
        dfs(root.as_ref(), 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3]);
        assert_eq!(25, Solution::sum_numbers(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[4, 9, 0, 5, 1]);
        assert_eq!(1026, Solution::sum_numbers(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[4, 9, 0, null, 1]);
        assert_eq!(531, Solution::sum_numbers(root));
    }
}
