use crate::utils::tree::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(nums: &[i32], i: &mut usize, bound: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if *i >= nums.len() || nums[*i] > bound {
                return None;
            }
            let n = nums[*i];
            let mut node = TreeNode::new(n);
            *i += 1;

            node.left = dfs(nums, i, n);
            node.right = dfs(nums, i, bound);
            Some(Rc::new(RefCell::new(node)))
        }

        let mut i = 0;
        dfs(&preorder, &mut i, i32::MAX)
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
        let expected = TreeNode::from_vec(&[1, null, 2, null, 3]);
        assert_eq!(
            expected.unwrap().borrow().to_string(),
            root.unwrap().borrow().to_string()
        );
    }
}
