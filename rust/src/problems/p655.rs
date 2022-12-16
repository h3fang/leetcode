pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        fn height(root: Option<&Rc<RefCell<TreeNode>>>) -> usize {
            match root {
                Some(n) => {
                    let l = height(n.borrow().left.as_ref());
                    let r = height(n.borrow().right.as_ref());
                    1 + l.max(r)
                }
                None => 0,
            }
        }
        let h = height(root.as_ref());
        let w = (1 << h) - 1;
        let mut result = vec![vec!["".to_string(); w]; h];
        fn dfs(
            root: Option<&Rc<RefCell<TreeNode>>>,
            result: &mut Vec<Vec<String>>,
            h: i32,
            row: i32,
            col: i32,
        ) {
            if let Some(node) = root {
                let n = node.borrow();
                result[row as usize][col as usize] = n.val.to_string();
                let p = 1 << (h - row - 1).clamp(0, 10);
                dfs(n.left.as_ref(), result, h, row + 1, col - p);
                dfs(n.right.as_ref(), result, h, row + 1, col + p);
            }
        }
        dfs(
            root.as_ref(),
            &mut result,
            h as i32 - 1,
            0,
            (w as i32 - 1) / 2,
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2]);
        let expected = [["", "1", ""], ["2", "", ""]];
        let expected = expected
            .iter()
            .map(|r| r.iter().map(|e| e.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::print_tree(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, 2, 3, null, 4]);
        let expected = [
            ["", "", "", "1", "", "", ""],
            ["", "2", "", "", "", "3", ""],
            ["", "", "4", "", "", "", ""],
        ];
        let expected = expected
            .iter()
            .map(|r| r.iter().map(|e| e.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::print_tree(root));
    }
}
