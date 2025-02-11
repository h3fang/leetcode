pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, curr: &mut Vec<u8>, min: &mut Vec<u8>) {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                curr.push(b'a' + n.val as u8);
                match (n.left.take(), n.right.take()) {
                    (None, None) => {
                        curr.reverse();
                        if min.is_empty() || curr < min {
                            min.clone_from(curr);
                        }
                        curr.reverse();
                    }
                    (None, Some(r)) => dfs(Some(r), curr, min),
                    (Some(l), None) => dfs(Some(l), curr, min),
                    (Some(l), Some(r)) => {
                        dfs(Some(l), curr, min);
                        dfs(Some(r), curr, min);
                    }
                }
                curr.pop();
            }
        }
        let mut min = vec![];
        dfs(root, &mut vec![], &mut min);
        unsafe { String::from_utf8_unchecked(min) }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[0, 1, 2, 3, 4, 3, 4]);
        assert_eq!("dba", Solution::smallest_from_leaf(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[25, 1, 3, 1, 3, 0, 2]);
        assert_eq!("adz", Solution::smallest_from_leaf(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[2, 2, 1, null, 1, 0, null, 0]);
        assert_eq!("abc", Solution::smallest_from_leaf(root));
    }
}
