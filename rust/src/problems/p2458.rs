pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        fn dfs1(root: Option<&Rc<RefCell<TreeNode>>>, h: &mut [i32]) -> i32 {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    if h[n.val as usize] == -1 {
                        let r = dfs1(n.left.as_ref(), h).max(dfs1(n.right.as_ref(), h)) + 1;
                        h[n.val as usize] = r;
                    }
                    h[n.val as usize]
                }
                None => 0,
            }
        }

        fn dfs2(
            root: Option<&Rc<RefCell<TreeNode>>>,
            mut depth: i32,
            h: i32,
            max: &mut [i32],
            heights: &[i32],
        ) {
            if let Some(node) = root {
                let n = node.borrow();
                depth += 1;
                max[n.val as usize] = h;
                let h_r = if let Some(n) = n.right.as_ref() {
                    heights[n.borrow().val as usize]
                } else {
                    0
                };
                dfs2(n.left.as_ref(), depth, h.max(depth + h_r), max, heights);
                let h_l = if let Some(n) = n.left.as_ref() {
                    heights[n.borrow().val as usize]
                } else {
                    0
                };
                dfs2(n.right.as_ref(), depth, h.max(depth + h_l), max, heights);
            }
        }

        let mut heights = [-1; 10_0001];
        dfs1(root.as_ref(), &mut heights);

        let mut max = [0; 10_0001];
        dfs2(root.as_ref(), -1, 0, &mut max, &heights);
        queries.into_iter().map(|q| max[q as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 3, 4, 2, null, 6, 5, null, null, null, null, null, 7]);
        let queries = vec![4];
        assert_eq!(vec![2], Solution::tree_queries(root, queries));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 8, 9, 2, 1, 3, 7, 4, 6]);
        let queries = vec![3, 2, 4, 8];
        assert_eq!(vec![3, 2, 3, 2], Solution::tree_queries(root, queries));
    }
}
