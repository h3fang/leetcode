pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn f(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            pre.first().map(|&val| {
                let n = pre.len();
                let (left, right) = if n == 1 {
                    (None, None)
                } else {
                    let i = post.iter().position(|e| *e == pre[1]).unwrap();
                    let left = f(&pre[1..i + 2], &post[..i + 1]);
                    let right = f(&pre[i + 2..], &post[i + 1..n - 1]);
                    (left, right)
                };
                Rc::new(RefCell::new(TreeNode { val, left, right }))
            })
        }
        f(&preorder, &postorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(
            expected,
            Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1])
        );
    }

    #[test]
    fn case2() {
        let expected = TreeNode::from_vec(&[1]);
        assert_eq!(
            expected,
            Solution::construct_from_pre_post(vec![1], vec![1])
        );
    }
}
