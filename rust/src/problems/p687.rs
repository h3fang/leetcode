pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
            match root {
                Some(node) => {
                    let mut n = node.borrow_mut();
                    let v = n.val;
                    let lv = n.left.as_ref().map(|n| n.borrow().val);
                    let rv = n.right.as_ref().map(|n| n.borrow().val);
                    let mut len = 1;
                    let l = dfs(n.left.take(), result);
                    let r = dfs(n.right.take(), result);
                    let mut max = 1;
                    if lv.is_some() && lv.unwrap() == v {
                        len += l;
                        max = 1 + l;
                    }
                    if rv.is_some() && rv.unwrap() == v {
                        len += r;
                        max = max.max(1 + r);
                    }
                    *result = (*result).max(len);
                    max
                }
                None => 0,
            }
        }
        let mut result = 0;
        dfs(root, &mut result);
        (result - 1).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 4, 5, 1, 1, 5]);
        assert_eq!(2, Solution::longest_univalue_path(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 4, 5, 4, 4, 5]);
        assert_eq!(2, Solution::longest_univalue_path(root));
    }
}
