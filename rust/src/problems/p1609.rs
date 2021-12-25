use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        let mut row = vec![root.unwrap()];
        let mut odd = false;
        while !row.is_empty() {
            if !row
                .iter()
                .all(|n| n.borrow().val % 2 == if odd { 0 } else { 1 })
            {
                return false;
            }
            if !row.windows(2).all(|w| {
                if odd {
                    w[1].borrow().val < w[0].borrow().val
                } else {
                    w[1].borrow().val > w[0].borrow().val
                }
            }) {
                return false;
            }
            row = row.iter().fold(vec![], |mut acc, n| {
                let mut n = n.borrow_mut();
                if let Some(left) = n.left.take() {
                    acc.push(left);
                }
                if let Some(right) = n.right.take() {
                    acc.push(right);
                }
                acc
            });
            odd = !odd;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 9, 1, 3, 5, 7]);
        assert_eq!(false, Solution::is_even_odd_tree(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[11, 8, 6, 1, 3, 9, 11, 30, 20, 18, 16, 12, 10, 4, 2, 17]);
        assert_eq!(true, Solution::is_even_odd_tree(root));
    }
}
