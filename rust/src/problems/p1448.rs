pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max: i32, result: &mut i32) {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                let v = n.val;
                if max <= v {
                    *result += 1;
                }
                let max = max.max(v);
                dfs(n.left.take(), max, result);
                dfs(n.right.take(), max, result);
            }
        }
        let mut result = 0;
        dfs(root, i32::MIN, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 1, 4, 3, null, 1, 5]);
        assert_eq!(4, Solution::good_nodes(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[3, 3, null, 4, 2]);
        assert_eq!(3, Solution::good_nodes(root));
    }
}
