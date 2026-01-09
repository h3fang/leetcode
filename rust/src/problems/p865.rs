pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (u32, Option<Rc<RefCell<TreeNode>>>) {
            let Some(root) = root else {
                return (0, None);
            };
            let (depth_l, l) = dfs(root.borrow().left.clone());
            let (depth_r, r) = dfs(root.borrow().right.clone());
            match depth_l.cmp(&depth_r) {
                std::cmp::Ordering::Less => (depth_r + 1, r),
                std::cmp::Ordering::Equal => (depth_r + 1, Some(root)),
                std::cmp::Ordering::Greater => (depth_l + 1, l),
            }
        }
        dfs(root).1
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
        let expected = TreeNode::from_vec(&[2, 7, 4]);
        assert_eq!(expected, Solution::subtree_with_all_deepest(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1]);
        let expected = TreeNode::from_vec(&[1]);
        assert_eq!(expected, Solution::subtree_with_all_deepest(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[0, 1, 3, null, 2]);
        let expected = TreeNode::from_vec(&[2]);
        assert_eq!(expected, Solution::subtree_with_all_deepest(root));
    }
}
