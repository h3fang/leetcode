use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                dfs(node.left.as_ref(), nodes);
                nodes.push(node.val);
                dfs(node.right.as_ref(), nodes);
            }
        }
        let mut n1 = vec![];
        let mut n2 = vec![];
        dfs(root1.as_ref(), &mut n1);
        dfs(root2.as_ref(), &mut n2);

        let mut i = 0;
        let mut j = 0;
        let mut result = Vec::with_capacity(n1.len() + n2.len());
        while i < n1.len() || j < n2.len() {
            if i < n1.len() && (j == n2.len() || n1[i] < n2[j]) {
                result.push(n1[i]);
                i += 1;
            } else {
                result.push(n2[j]);
                j += 1;
            }
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
        let root1 = TreeNode::from_vec(&[2, 1, 4]);
        let root2 = TreeNode::from_vec(&[1, 0, 3]);
        let result = Solution::get_all_elements(root1, root2);
        assert_eq!(vec![0, 1, 1, 2, 3, 4], result);
    }

    #[test]
    fn case2() {
        let root1 = TreeNode::from_vec(&[1, null, 8]);
        let root2 = TreeNode::from_vec(&[8, 1]);
        let result = Solution::get_all_elements(root1, root2);
        assert_eq!(vec![1, 1, 8, 8], result);
    }
}
