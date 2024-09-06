pub struct Solution;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, k: i32, targets: &mut HashSet<i32>) -> bool {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    if targets.contains(&n.val) {
                        true
                    } else {
                        targets.insert(k - n.val);
                        dfs(n.left.as_ref(), k, targets) || dfs(n.right.as_ref(), k, targets)
                    }
                }
                None => false,
            }
        }
        let mut targets = HashSet::new();
        dfs(root.as_ref(), k, &mut targets)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 3, 6, 2, 4, null, 7]);
        assert!(Solution::find_target(root, 9));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 3, 6, 2, 4, null, 7]);
        assert!(!Solution::find_target(root, 28));
    }
}
