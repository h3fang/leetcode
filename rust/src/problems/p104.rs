use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    dfs(n.left.as_ref()).max(dfs(n.right.as_ref())) + 1
                }
            }
        }
        dfs(root.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        assert_eq!(3, Solution::max_depth(root));
    }
}
