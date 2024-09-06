pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> String {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    match (&n.left, &n.right) {
                        (None, None) => n.val.to_string(),
                        (None, Some(r)) => {
                            format!("{}()({})", n.val, dfs(Some(r)))
                        }
                        (Some(l), None) => {
                            format!("{}({})", n.val, dfs(Some(l)))
                        }
                        (Some(l), Some(r)) => {
                            format!("{}({})({})", n.val, dfs(Some(l)), dfs(Some(r)))
                        }
                    }
                }
                _ => String::new(),
            }
        }
        dfs(root.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4]);
        assert_eq!("1(2(4))(3)", Solution::tree2str(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 3, null, 4]);
        assert_eq!("1(2()(4))(3)", Solution::tree2str(root));
    }
}
