use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let r = match n.left.as_ref() {
                        None => 0,
                        Some(left) => {
                            let left = left.borrow();
                            if left.left.is_none() && left.right.is_none() {
                                left.val
                            } else {
                                0
                            }
                        }
                    };
                    r + dfs(n.right.as_ref()) + dfs(n.left.as_ref())
                }
                None => 0,
            }
        }
        dfs(root.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::null;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        assert_eq!(24, Solution::sum_of_left_leaves(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[3]);
        assert_eq!(0, Solution::sum_of_left_leaves(root));
    }
}
