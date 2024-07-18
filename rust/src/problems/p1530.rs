pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, f: &mut Vec<i32>, d: usize, distance: usize) {
            if d >= distance {
                return;
            }
            if let Some(node) = root {
                let n = node.borrow();
                if n.left.is_none() && n.right.is_none() {
                    f[d] += 1;
                }
                dfs(n.left.as_ref(), f, d + 1, distance);
                dfs(n.right.as_ref(), f, d + 1, distance);
            }
        }

        let d = distance as usize;
        match root {
            Some(node) => {
                let mut result = 0;
                let mut n = node.borrow_mut();
                if n.left.is_some() && n.right.is_some() {
                    let mut l = vec![0; d + 1];
                    dfs(n.left.as_ref(), &mut l, 1, d);
                    let mut r = vec![0; d + 1];
                    dfs(n.right.as_ref(), &mut r, 1, d);
                    for (i, &x) in l.iter().enumerate() {
                        for (j, &y) in r.iter().enumerate() {
                            if i + j <= d {
                                result += x * y;
                            }
                        }
                    }
                }
                result
                    + Self::count_pairs(n.left.take(), distance)
                    + Self::count_pairs(n.right.take(), distance)
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, null, 4]);
        assert_eq!(1, Solution::count_pairs(root, 3));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(2, Solution::count_pairs(root, 3));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[7, 1, 4, 6, null, 5, 3, null, null, null, null, null, 2]);
        assert_eq!(1, Solution::count_pairs(root, 3));
    }
}
