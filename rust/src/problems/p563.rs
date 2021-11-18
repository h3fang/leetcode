use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let left = helper(n.left.as_ref(), result);
                    let right = helper(n.right.as_ref(), result);
                    *result += (left - right).abs();
                    left + right + n.val
                }
                None => 0,
            }
        }
        let mut result = 0;
        helper(root.as_ref(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3]);
        assert_eq!(1, Solution::find_tilt(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[4, 2, 9, 3, 5, null, 7]);
        assert_eq!(15, Solution::find_tilt(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[21, 7, 14, 1, 1, 2, 2, 3, 3]);
        assert_eq!(9, Solution::find_tilt(root));
    }
}
