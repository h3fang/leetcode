pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let (l, d_l) = dfs(n.left.clone());
                    let (r, d_r) = dfs(n.right.clone());
                    drop(n);
                    match d_l.cmp(&d_r) {
                        std::cmp::Ordering::Less => (r, d_r + 1),
                        std::cmp::Ordering::Equal => (Some(node), d_l + 1),
                        std::cmp::Ordering::Greater => (l, d_l + 1),
                    }
                }
                None => (None, 0),
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
        let root = TreeNode::from_vec(&[3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
        let expected = TreeNode::from_vec(&[2, 7, 4]);
        assert_eq!(expected, Solution::lca_deepest_leaves(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1]);
        let expected = TreeNode::from_vec(&[1]);
        assert_eq!(expected, Solution::lca_deepest_leaves(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[0, 1, 3, null, 2]);
        let expected = TreeNode::from_vec(&[2]);
        assert_eq!(expected, Solution::lca_deepest_leaves(root));
    }
}
