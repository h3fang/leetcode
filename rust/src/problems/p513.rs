pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, depth: i32, bl: &mut (i32, i32)) {
            if let Some(node) = root {
                let n = node.borrow();
                if depth > bl.1 {
                    bl.0 = n.val;
                    bl.1 = depth;
                }
                dfs(n.left.as_ref(), depth + 1, bl);
                dfs(n.right.as_ref(), depth + 1, bl);
            }
        }
        let mut bl = (-1, -1);
        dfs(root.as_ref(), 0, &mut bl);
        bl.0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        assert_eq!(1, Solution::find_bottom_left_value(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, null, 5, 6, null, null, 7]);
        assert_eq!(7, Solution::find_bottom_left_value(root));
    }
}
