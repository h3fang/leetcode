use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            match root {
                None => {}
                Some(node) => {
                    let n = node.borrow();
                    dfs(n.left.as_ref(), result);
                    result.push(n.val);
                    dfs(n.right.as_ref(), result);
                }
            }
        }
        dfs(root.as_ref(), &mut result);
        result
    }

    #[allow(clippy::assigning_clones)]
    pub fn inorder_traversal_iterative(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = Vec::new();
        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                stack.push(Some(Rc::clone(&node)));
                root = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop().unwrap() {
                let n = node.borrow();
                result.push(n.val);
                root.clone_from(&n.right);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::null;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, null, 2, 3]);
        let expected = vec![1, 3, 2];
        assert_eq!(expected, Solution::inorder_traversal(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[]);
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, Solution::inorder_traversal(root));
    }

    #[test]
    fn case3() {
        let root = TreeNode::from_vec(&[1]);
        let expected = vec![1];
        assert_eq!(expected, Solution::inorder_traversal(root));
    }

    #[test]
    fn case4() {
        let root = TreeNode::from_vec(&[1, 2]);
        let expected = vec![2, 1];
        assert_eq!(expected, Solution::inorder_traversal(root));
    }

    #[test]
    fn case5() {
        let root = TreeNode::from_vec(&[1, null, 2]);
        let expected = vec![1, 2];
        assert_eq!(expected, Solution::inorder_traversal(root));
    }
}
