pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let t = traversal.as_bytes();
        let mut s: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::with_capacity(64);
        s.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        let (n, mut i) = (traversal.len(), 0);
        while i < n {
            let mut depth = 0;
            while i < n && t[i] == b'-' {
                depth += 1;
                i += 1;
            }
            let mut val = 0;
            while i < n && t[i] != b'-' {
                val = val * 10 + (t[i] - b'0') as i32;
                i += 1;
            }
            while s.len() > depth {
                s.pop();
            }
            let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if depth > 0 {
                let p = s[depth - 1].as_mut().unwrap();
                let mut p = p.borrow_mut();
                if p.left.is_none() {
                    p.left = node.clone();
                } else {
                    p.right = node.clone();
                }
            }
            if depth < s.len() {
                s[depth] = node.clone();
            } else {
                s.push(node);
            }
        }
        s.into_iter().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let expected = TreeNode::from_vec(&[1, 2, 5, 3, 4, 6, 7]);
        assert_eq!(
            expected,
            Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string())
        );
    }

    #[test]
    fn case2() {
        let expected = TreeNode::from_vec(&[1, 2, 5, 3, null, 6, null, 4, null, 7]);
        assert_eq!(
            expected,
            Solution::recover_from_preorder("1-2--3---4-5--6---7".to_string())
        );
    }

    #[test]
    fn case3() {
        let expected = TreeNode::from_vec(&[1, 401, null, 349, 88, 90]);
        assert_eq!(
            expected,
            Solution::recover_from_preorder("1-401--349---90--88".to_string())
        );
    }
}
