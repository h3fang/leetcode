use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn d_left(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut d = 0;
            while let Some(node) = root {
                d += 1;
                root = node.borrow().left.clone();
            }
            d
        }

        fn d_right(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut d = 0;
            while let Some(node) = root {
                d += 1;
                root = node.borrow().right.clone();
            }
            d
        }
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, d_l: Option<i32>, d_r: Option<i32>) -> i32 {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let left = d_l.unwrap_or_else(|| d_left(n.left.clone()));
                    let right = d_r.unwrap_or_else(|| d_right(n.right.clone()));
                    if left == right {
                        2i32.pow(left as u32 + 1) - 1
                    } else {
                        1 + dfs(n.left.as_ref(), Some(left - 1), None)
                            + dfs(n.right.as_ref(), None, Some(right - 1))
                    }
                }
                None => 0,
            }
        }

        dfs(root.as_ref(), None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(6, Solution::count_nodes(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[]);
        assert_eq!(0, Solution::count_nodes(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1]);
        assert_eq!(1, Solution::count_nodes(root));
    }
}
