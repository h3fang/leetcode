pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, depth: usize, rs: &mut Vec<i32>) {
            if let Some(node) = root {
                let n = node.borrow();
                if depth + 1 > rs.len() {
                    rs.push(0);
                }
                rs[depth] = n.val;
                dfs(n.left.as_ref(), depth + 1, rs);
                dfs(n.right.as_ref(), depth + 1, rs);
            }
        }
        let mut result = vec![];
        dfs(root.as_ref(), 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, null, 5, null, 4]);
        assert_eq!(vec![1, 3, 4], Solution::right_side_view(root));
    }
}
