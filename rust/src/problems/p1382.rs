pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                dfs(n.left.take(), nodes);
                nodes.push(n.val);
                dfs(n.right.take(), nodes);
            }
        }
        let mut nodes = vec![];
        dfs(root, &mut nodes);

        fn build(nodes: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let n = nodes.len();
            if n == 0 {
                return None;
            }
            if n == 1 {
                return Some(Rc::new(RefCell::new(TreeNode::new(nodes[0]))));
            }
            let left = build(&nodes[..n / 2]);
            let right = build(&nodes[n / 2 + 1..]);
            Some(Rc::new(RefCell::new(TreeNode {
                val: nodes[n / 2],
                left,
                right,
            })))
        }
        build(&nodes)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, null, 2, null, 3, null, 4, null, null]);
        let expected = [
            TreeNode::from_vec(&[2, 1, 3, null, null, null, 4]),
            TreeNode::from_vec(&[2, 1, 4, null, null, 3]),
            TreeNode::from_vec(&[3, 1, 4, null, 2]),
            TreeNode::from_vec(&[3, 2, 4, 1]),
        ];
        assert!(expected.contains(&Solution::balance_bst(root)));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        let expected = TreeNode::from_vec(&[2, 1, 3]);
        assert_eq!(expected, Solution::balance_bst(root));
    }
}
