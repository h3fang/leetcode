pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            to_delete: &HashSet<i32>,
            result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                Some(node) => {
                    let mut n = node.borrow_mut();
                    if to_delete.contains(&n.val) {
                        let l = dfs(n.left.take(), to_delete, result);
                        if l.is_some() {
                            result.push(l);
                        }
                        let r = dfs(n.right.take(), to_delete, result);
                        if r.is_some() {
                            result.push(r);
                        }
                        None
                    } else {
                        n.left = dfs(n.left.take(), to_delete, result);
                        n.right = dfs(n.right.take(), to_delete, result);
                        drop(n);
                        Some(node)
                    }
                }
                None => None,
            }
        }
        let to_delete = to_delete.into_iter().collect();
        let mut result = vec![];
        let root = dfs(root, &to_delete, &mut result);
        if root.is_some() {
            result.push(root);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6, 7]);
        let to_delete = vec![3, 5];
        let expected = [vec![1, 2, null, 4], vec![6], vec![7]]
            .iter()
            .map(|nums| TreeNode::from_vec(nums))
            .collect::<Vec<_>>();
        let result = Solution::del_nodes(root, to_delete);
        assert!(result.iter().all(|t| expected.contains(t)));
    }
}
