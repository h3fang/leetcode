use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(n) = root {
                let n = n.borrow();
                result.push(n.val);
                dfs(n.left.as_ref(), result);
                dfs(n.right.as_ref(), result);
            }
        }
        let mut r = vec![];
        dfs(root.as_ref(), &mut r);
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, null, 2, 3]);
        assert_eq!(vec![1, 2, 3], Solution::preorder_traversal(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[]);
        assert_eq!(vec![0i32; 0], Solution::preorder_traversal(root));
    }
}
