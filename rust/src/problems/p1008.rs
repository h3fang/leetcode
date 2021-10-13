use crate::utils::tree::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &mut TreeNode, nums: &[i32], i: &mut usize, stack: &mut Vec<i32>) {
            if let Some(n1) = nums.get(*i) {
                if *n1 < node.val {
                    let mut left = TreeNode::new(*n1);
                    stack.push(node.val);
                    *i += 1;
                    dfs(&mut left, nums, i, stack);
                    node.left = Some(Rc::new(RefCell::new(left)));
                    stack.pop();

                    if let Some(n2) = nums.get(*i) {
                        if stack.is_empty() || (n2 > &node.val && n2 < stack.last().unwrap()) {
                            let mut right = TreeNode::new(*n2);
                            *i += 1;
                            dfs(&mut right, nums, i, stack);
                            node.right = Some(Rc::new(RefCell::new(right)));
                        }
                    }
                } else if stack.is_empty() || n1 < stack.last().unwrap() {
                    let mut right = TreeNode::new(*n1);
                    *i += 1;
                    dfs(&mut right, nums, i, stack);
                    node.right = Some(Rc::new(RefCell::new(right)));
                }
            }
        }
        let mut stack = vec![];
        let root = preorder.first().map(|n| {
            let mut node = TreeNode::new(*n);
            let mut i = 1;
            dfs(&mut node, &preorder, &mut i, &mut stack);
            Rc::new(RefCell::new(node))
        });
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let preorder = vec![8, 5, 1, 7, 10, 12];
        let root = Solution::bst_from_preorder(preorder);
        let expected = TreeNode::from_vec(&[8, 5, 10, 1, 7, null, 12]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }

    #[test]
    fn case2() {
        let preorder = vec![1, 2, 3];
        let root = Solution::bst_from_preorder(preorder);
        let expected = TreeNode::from_vec(&[1, null, 2, null, null, null, 3]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }
}
