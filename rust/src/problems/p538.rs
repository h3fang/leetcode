pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> i32 {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                sum = dfs(&mut n.right, sum);
                let v = n.val;
                n.val += sum;
                sum = dfs(&mut n.left, sum + v);
            }
            sum
        }
        dfs(&mut root, 0);
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[
            4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8,
        ]);
        let expected = TreeNode::from_vec(&[
            30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8,
        ]);
        assert_eq!(expected, Solution::convert_bst(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[0, null, 1]);
        let expected = TreeNode::from_vec(&[1, null, 1]);
        assert_eq!(expected, Solution::convert_bst(root));
    }
}
