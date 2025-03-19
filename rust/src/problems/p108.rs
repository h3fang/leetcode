use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }
            let root = nums.len() / 2;
            let left = helper(&nums[..root]);
            let right = helper(&nums[root + 1..]);
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[root],
                left,
                right,
            })))
        }

        helper(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::{LeetCodeTreeNodes, null};

    #[test]
    fn case1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let ltn = LeetCodeTreeNodes {
            nums: vec![0, -3, 9, -10, null, 5],
        };
        let result = Solution::sorted_array_to_bst(nums);
        assert_eq!(ltn.to_string(), result.unwrap().borrow().to_string());
    }
}
