pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> (bool, i32, i32, i32) {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let l = dfs(n.left.as_ref(), result);
                    let r = dfs(n.right.as_ref(), result);
                    if l.0 && r.0 && l.2 < n.val && r.1 > n.val {
                        let sum = l.3 + r.3 + n.val;
                        *result = (*result).max(sum);
                        (true, l.1.min(n.val), r.2.max(n.val), sum)
                    } else {
                        (false, 0, 0, 0)
                    }
                }
                None => (true, i32::MAX, i32::MIN, 0),
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
        let root = TreeNode::from_vec(&[
            1, 4, 3, 2, 4, 2, 5, null, null, null, null, null, null, 4, 6,
        ]);
        assert_eq!(20, Solution::max_sum_bst(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[4, 3, null, 1, 2]);
        assert_eq!(2, Solution::max_sum_bst(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        assert_eq!(6, Solution::max_sum_bst(root));
    }

    #[test]
    fn case4() {
        let root = TreeNode::from_vec(&[5, 4, 8, 3, null, 6, 3]);
        assert_eq!(7, Solution::max_sum_bst(root));
    }
}
