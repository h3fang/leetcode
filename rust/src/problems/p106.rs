use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            postorder.last().map(|&root| {
                let n = inorder.len();
                let mut i = 0;
                while i < n {
                    if inorder[i] == root {
                        break;
                    }
                    i += 1;
                }
                let left = helper(&inorder[..i], &postorder[..i]);
                let right = helper(&inorder[i + 1..], &postorder[i..n - 1]);
                Rc::new(RefCell::new(TreeNode {
                    val: root,
                    left,
                    right,
                }))
            })
        }

        helper(&inorder, &postorder)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let expected = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        let result = Solution::build_tree(inorder, postorder);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case2() {
        let inorder = vec![-1];
        let postorder = vec![-1];
        let expected = TreeNode::from_vec(&[-1]);
        let result = Solution::build_tree(inorder, postorder);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            result.unwrap().borrow().to_string()
        );
    }
}
