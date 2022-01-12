use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn insert_into_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
            match root {
                Some(node) => {
                    let mut n = node.borrow_mut();
                    if n.val < val {
                        dfs(&mut n.right, val);
                    } else {
                        dfs(&mut n.left, val);
                    }
                }
                None => {
                    *root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            }
        }
        dfs(&mut root, val);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[4, 2, 7, 1, 3]);
        let result = Solution::insert_into_bst(root, 5);
        let expected = TreeNode::from_vec(&[4, 2, 7, 1, 3, 5]);
        assert_eq!(expected, result);
    }
}
