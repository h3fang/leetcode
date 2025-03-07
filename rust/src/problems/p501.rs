pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            curr: &mut i32,
            streak: &mut i32,
            max: &mut i32,
            result: &mut Vec<i32>,
        ) {
            if let Some(node) = root {
                let mut node = node.borrow_mut();
                dfs(node.left.take(), curr, streak, max, result);
                if node.val == *curr {
                    *streak += 1;
                } else {
                    *curr = node.val;
                    *streak = 1;
                }
                match (*streak).cmp(max) {
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => result.push(node.val),
                    std::cmp::Ordering::Greater => {
                        *max = *streak;
                        result.clear();
                        result.push(node.val);
                    }
                }
                dfs(node.right.take(), curr, streak, max, result);
            }
        }
        let (mut curr, mut result) = (i32::MIN, vec![]);
        dfs(root, &mut curr, &mut 0, &mut 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2],
            Solution::find_mode(TreeNode::from_vec(&[1, null, 2, 2]))
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0], Solution::find_mode(TreeNode::from_vec(&[0])));
    }
}
