use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                dfs(n.left.take(), nums);
                nums.push(n.val);
                dfs(n.right.take(), nums);
            }
        }
        let mut nums = vec![];
        dfs(root, &mut nums);
        nums.windows(2).map(|w| w[1] - w[0]).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[4, 2, 6, 1, 3]);
        assert_eq!(1, Solution::min_diff_in_bst(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 0, 48, null, null, 12, 49]);
        assert_eq!(1, Solution::min_diff_in_bst(root));
    }
}
