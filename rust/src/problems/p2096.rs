use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, value: i32) -> String {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    if n.val == value {
                        "H".into()
                    } else {
                        let mut left = dfs(n.left.as_ref(), value);
                        if !left.is_empty() {
                            left.push('L');
                            return left;
                        }
                        let mut right = dfs(n.right.as_ref(), value);
                        if !right.is_empty() {
                            right.push('R');
                            return right;
                        }
                        "".into()
                    }
                }
                None => "".into(),
            }
        }
        let start = dfs(root.as_ref(), start_value)
            .chars()
            .rev()
            .collect::<String>();
        let dest = dfs(root.as_ref(), dest_value)
            .chars()
            .rev()
            .collect::<String>();
        let mut i = 0;
        while start.as_bytes()[i] == dest.as_bytes()[i] {
            if i + 1 == start.len() || i + 1 == dest.len() {
                break;
            }
            i += 1;
        }
        "U".repeat(start.len() - 1 - i) + &dest[i..dest.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 1, 2, 3, null, 6, 4]);
        assert_eq!("UURL".to_string(), Solution::get_directions(root, 3, 6));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[2, 1]);
        assert_eq!("L".to_string(), Solution::get_directions(root, 2, 1));
    }
}
