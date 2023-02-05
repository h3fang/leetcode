pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, _n: i32, x: i32) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut [i32], x: i32, flag: u8) {
            if let Some(r) = root {
                if r.borrow().val == x {
                    let mut r = r.borrow_mut();
                    dfs(r.left.take(), ans, x, 1);
                    dfs(r.right.take(), ans, x, 2);
                } else {
                    ans[flag as usize] += 1;
                    let mut r = r.borrow_mut();
                    dfs(r.left.take(), ans, x, flag);
                    dfs(r.right.take(), ans, x, flag);
                }
            }
        }
        let mut a = [0; 3];
        dfs(root, &mut a, x, 0);
        let sum: i32 = a.iter().sum();
        a.iter().any(|&i| i > sum - i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert!(Solution::btree_game_winning_move(root, 11, 3));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 3]);
        assert!(!Solution::btree_game_winning_move(root, 3, 1));
    }
}
