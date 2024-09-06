use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, lb: Option<i32>, ub: Option<i32>) -> bool {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    if let Some(lb) = lb {
                        if n.val <= lb {
                            return false;
                        }
                    }

                    if let Some(ub) = ub {
                        if n.val >= ub {
                            return false;
                        }
                    }

                    return dfs(n.left.as_ref(), lb, Some(n.val))
                        && dfs(n.right.as_ref(), Some(n.val), ub);
                }
                None => true,
            }
        }

        dfs(root.as_ref(), None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::null;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 1, 4, null, null, 3, 6]);
        assert!(!Solution::is_valid_bst(root));
    }
}
