use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            preorder.first().map(|&root| {
                let mut i = 0;
                while i < inorder.len() {
                    if inorder[i] == root {
                        break;
                    }
                    i += 1;
                }
                let left = helper(&preorder[1..=i], &inorder[..i]);
                let right = helper(&preorder[i + 1..], &inorder[i + 1..]);
                Rc::new(RefCell::new(TreeNode {
                    val: root,
                    left,
                    right,
                }))
            })
        }

        helper(&preorder, &inorder)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let expected = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        let result = Solution::build_tree(preorder, inorder);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let expected = TreeNode::from_vec(&[-1]);
        let result = Solution::build_tree(preorder, inorder);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }
}
