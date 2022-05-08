pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32) {
            if let Some(node) = root {
                let v = node.borrow().val;
                let (sum_l, n_l) = dfs(node.borrow().left.as_ref(), result);
                let (sum_r, n_r) = dfs(node.borrow().right.as_ref(), result);
                let n = 1 + n_l + n_r;
                let sum = v + sum_l + sum_r;
                if sum / n == v {
                    *result += 1;
                }
                (sum, n)
            } else {
                (0, 0)
            }
        }
        let mut result = 0;
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
        let root = TreeNode::from_vec(&[4, 8, 5, 0, 1, null, 6]);
        assert_eq!(5, Solution::average_of_subtree(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1]);
        assert_eq!(1, Solution::average_of_subtree(root));
    }
}
