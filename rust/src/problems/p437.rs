use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let n = node.borrow();
                    let r = i32::from(n.val == target_sum);
                    r + dfs(n.left.as_ref(), target_sum - n.val)
                        + dfs(n.right.as_ref(), target_sum - n.val)
                }
            }
        }

        dfs(root.as_ref(), target_sum)
            + match root {
                None => 0,
                Some(node) => {
                    let mut n = node.borrow_mut();
                    Self::path_sum(n.left.take(), target_sum)
                        + Self::path_sum(n.right.take(), target_sum)
                }
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::null;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[10, 5, -3, 3, 2, null, 11, 3, -2, null, 1]);
        assert_eq!(3, Solution::path_sum(root, 8));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1]);
        assert_eq!(3, Solution::path_sum(root, 22));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1, null, 2, null, 3, null, 4, null, 5]);
        assert_eq!(2, Solution::path_sum(root, 3));
    }
}
