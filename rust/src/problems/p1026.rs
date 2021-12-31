use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, minmax: Option<(i32, i32)>) -> i32 {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let r = if let Some((min, max)) = minmax {
                        (n.val - min).abs().max((n.val - max).abs())
                    } else {
                        0
                    };
                    let minmax = if let Some((min, max)) = minmax {
                        Some((min.min(n.val), max.max(n.val)))
                    } else {
                        Some((n.val, n.val))
                    };
                    r.max(dfs(n.left.as_ref(), minmax).max(dfs(n.right.as_ref(), minmax)))
                }
                None => 0,
            }
        }
        dfs(root.as_ref(), None)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[8, 3, 10, 1, 6, null, 14, null, null, 4, 7, 13]);
        assert_eq!(7, Solution::max_ancestor_diff(root));
    }
}
