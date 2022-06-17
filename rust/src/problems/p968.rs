pub struct Solution;
use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            match root {
                Some(node) => {
                    let mut n = node.borrow_mut();
                    let l = dfs(n.left.take());
                    let r = dfs(n.right.take());
                    let minl = l.1.min(l.2);
                    let minr = r.1.min(r.2);
                    (
                        l.1 + r.1,
                        (l.2 + minr).min(r.2 + minl),
                        1 + l.0.min(minl) + r.0.min(minr),
                    )
                }
                None => (0, 0, i32::MAX / 4),
            }
        }
        let r = dfs(root);
        r.1.min(r.2)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[0, 0, null, 0, 0]);
        assert_eq!(1, Solution::min_camera_cover(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[0, 0, null, 0, null, 0, null, null, 0]);
        assert_eq!(2, Solution::min_camera_cover(root));
    }
}
