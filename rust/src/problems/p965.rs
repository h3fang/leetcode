pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> bool {
            match root {
                Some(node) => {
                    node.borrow().val == val
                        && dfs(node.borrow().left.as_ref(), val)
                        && dfs(node.borrow().right.as_ref(), val)
                }
                None => true,
            }
        }

        if let Some(r) = &root {
            dfs(root.as_ref(), r.borrow().val)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 1, 1, 1, 1, null, 1]);
        assert_eq!(true, Solution::is_unival_tree(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[2, 2, 2, 5, 2]);
        assert_eq!(false, Solution::is_unival_tree(root));
    }
}
