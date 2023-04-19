use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn zigzag(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32) {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let a = zigzag(n.left.as_ref(), result);
                    let b = zigzag(n.right.as_ref(), result);
                    let r = (a.1 + 1, b.0 + 1);
                    (*result) = (*result).max(r.0.max(r.1));
                    r
                }
                None => (-1, -1),
            }
        }
        let mut result = 0;
        zigzag(root.as_ref(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[
            1, null, 1, 1, 1, null, null, 1, 1, null, 1, null, null, null, 1, null, 1,
        ]);
        assert_eq!(3, Solution::longest_zig_zag(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 1, 1, null, 1, null, null, 1, 1, null, 1]);
        assert_eq!(4, Solution::longest_zig_zag(root));
    }
}
