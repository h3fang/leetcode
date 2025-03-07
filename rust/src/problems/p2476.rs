pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut tree = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, tree: &mut Vec<i32>) {
            if let Some(r) = root {
                let mut r = r.borrow_mut();
                dfs(r.left.take(), tree);
                tree.push(r.val);
                dfs(r.right.take(), tree);
            }
        }
        dfs(root, &mut tree);
        queries
            .into_iter()
            .map(|q| {
                let i = tree.partition_point(|&x| x <= q);
                let a = if i == 0 { -1 } else { tree[i - 1] };
                let i = tree.partition_point(|&x| x < q);
                let b = if i == tree.len() { -1 } else { tree[i] };
                vec![a, b]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[
            6, 2, 13, 1, 4, 9, 15, null, null, null, null, null, null, 14,
        ]);
        let queries = vec![2, 5, 16];
        let expected = [[2, 2], [4, 6], [15, -1]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::closest_nodes(root, queries));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[4, null, 9]);
        let queries = vec![3];
        let expected = [[-1, 4]].iter().map(|e| e.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::closest_nodes(root, queries));
    }
}
