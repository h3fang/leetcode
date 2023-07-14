pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                Some(node) => {
                    let mut n = node.borrow_mut();
                    let (c_l, v_l) = dfs(n.left.take());
                    let (c_r, v_r) = dfs(n.right.take());
                    let v = v_l + v_r + 1 - n.val;
                    (c_l + c_r + v.abs(), v)
                }
                None => (0, 0),
            }
        }
        dfs(root).0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 0, 0]);
        assert_eq!(2, Solution::distribute_coins(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[0, 3, 0]);
        assert_eq!(3, Solution::distribute_coins(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1, 0, 2]);
        assert_eq!(2, Solution::distribute_coins(root));
    }

    #[test]
    fn case4() {
        let root = TreeNode::from_vec(&[1, 0, 0, null, 3]);
        assert_eq!(4, Solution::distribute_coins(root));
    }
}
