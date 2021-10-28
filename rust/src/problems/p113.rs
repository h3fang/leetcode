use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            root: Option<&Rc<RefCell<TreeNode>>>,
            sum: i32,
            target: i32,
            curr: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            match root {
                None => {}
                Some(node) => {
                    let n = node.borrow();
                    curr.push(n.val);
                    if n.left.is_none() && n.right.is_none() && sum + n.val == target {
                        result.push(curr.clone());
                    } else {
                        dfs(n.left.as_ref(), sum + n.val, target, curr, result);
                        dfs(n.right.as_ref(), sum + n.val, target, curr, result);
                    }
                    curr.pop();
                }
            }
        }

        let mut result = Vec::new();
        let mut curr = Vec::new();
        dfs(root.as_ref(), 0, target_sum, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1]);
        let mut result = Solution::path_sum(root, 22);
        result.sort_unstable();
        let mut expected = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 3]);
        let mut result = Solution::path_sum(root, 5);
        result.sort_unstable();
        let mut expected: Vec<Vec<i32>> = vec![];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
