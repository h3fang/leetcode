use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                let mut node = node.borrow_mut();
                dfs(node.left.take(), result);
                dfs(node.right.take(), result);
                if node.left.is_none() && node.right.is_none() {
                    result.push(node.val);
                }
            }
        }

        fn leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut r = Vec::new();
            dfs(root, &mut r);
            r
        }

        leaves(root1) == leaves(root2)
    }
}
