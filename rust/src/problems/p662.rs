pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut result = 1;
            let mut level = vec![(root, 0i32)];
            while !level.is_empty() {
                let mut next = vec![];
                result = result.max(level[level.len() - 1].1 - level[0].1 + 1);
                for (node, k) in level {
                    let mut node = node.borrow_mut();
                    if let Some(n) = node.left.take() {
                        next.push((n, 2 * k));
                    }
                    if let Some(n) = node.right.take() {
                        next.push((n, 2 * k + 1));
                    }
                }
                level = next;
            }
            result
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 3, 2, 5, 3, null, 9]);
        assert_eq!(4, Solution::width_of_binary_tree(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 3, null, 5, 3]);
        assert_eq!(2, Solution::width_of_binary_tree(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1, 3, 2, 5]);
        assert_eq!(2, Solution::width_of_binary_tree(root));
    }
}
