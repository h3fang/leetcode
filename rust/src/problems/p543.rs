use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                Some(n) => {
                    let n = n.borrow_mut();
                    let (depth_left, diameter_left) = dfs(&n.left);
                    let (depth_right, diameter_right) = dfs(&n.right);
                    let depth = depth_left.max(depth_right) + 1;
                    let diameter = diameter_left
                        .max(diameter_right)
                        .max(depth_left + depth_right);
                    (depth, diameter)
                }
                None => (0, 0),
            }
        }
        dfs(&root).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(3, result);
    }
}
