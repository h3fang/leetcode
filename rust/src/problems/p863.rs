pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

use std::collections::HashMap;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn distance_k(root: Option<Node>, target: Option<Node>, k: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        fn dfs_parent(
            root: Option<Node>,
            parent: Option<Node>,
            m: &mut HashMap<i32, Option<Node>>,
        ) {
            if let Some(node) = root {
                m.insert(node.borrow().val, parent);
                dfs_parent(node.borrow().left.clone(), Some(node.clone()), m);
                dfs_parent(node.borrow().right.clone(), Some(node.clone()), m);
            }
        }
        dfs_parent(root, None, &mut m);
        fn dfs(
            root: Option<Node>,
            parent: Option<Node>,
            m: &HashMap<i32, Option<Node>>,
            result: &mut Vec<i32>,
            k: i32,
        ) {
            if let Some(node) = root {
                if k == 0 {
                    result.push(node.borrow().val);
                    return;
                }
                let left = node.borrow_mut().left.take();
                if left != parent {
                    dfs(left, Some(node.clone()), m, result, k - 1);
                }
                let right = node.borrow_mut().right.take();
                if right != parent {
                    dfs(right, Some(node.clone()), m, result, k - 1);
                }
                let p = m.get(&node.borrow().val).unwrap().clone();
                if p != parent {
                    dfs(p, Some(node), m, result, k - 1);
                }
            }
        }
        let mut result = vec![];
        dfs(target, None, &m, &mut result, k);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::{find_node, null};

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
        let target = find_node(5, root.as_ref());
        let mut result = Solution::distance_k(root, target, 2);
        result.sort_unstable();
        assert_eq!(vec![1, 4, 7], result);
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1]);
        let target = find_node(1, root.as_ref());
        let result = Solution::distance_k(root, target, 3);
        assert!(result.is_empty());
    }
}
