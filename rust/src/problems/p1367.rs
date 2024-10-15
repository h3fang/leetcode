pub struct Solution;

use crate::utils::linked_list::ListNode;
use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        #[allow(clippy::borrowed_box)]
        fn dfs(h: Option<&Box<ListNode>>, root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match h {
                Some(h) => match root {
                    Some(r) => {
                        let r = r.borrow();
                        r.val == h.val
                            && (dfs(h.next.as_ref(), r.left.as_ref())
                                || dfs(h.next.as_ref(), r.right.as_ref()))
                    }
                    None => false,
                },
                None => true,
            }
        }

        let mut s = Vec::with_capacity(2500);
        s.push(root);
        while let Some(root) = s.pop() {
            if dfs(head.as_ref(), root.as_ref()) {
                return true;
            }
            if let Some(r) = root {
                let mut r = r.borrow_mut();
                if r.left.is_some() {
                    s.push(r.left.take());
                }
                if r.right.is_some() {
                    s.push(r.right.take());
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[4, 2, 8]);
        let root = TreeNode::from_vec(&[
            1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3,
        ]);
        assert!(Solution::is_sub_path(head, root));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 4, 2, 6]);
        let root = TreeNode::from_vec(&[
            1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3,
        ]);
        assert!(Solution::is_sub_path(head, root));
    }

    #[test]
    fn case3() {
        let head = ListNode::from_vec(&[1, 4, 2, 6, 8]);
        let root = TreeNode::from_vec(&[
            1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3,
        ]);
        assert!(!Solution::is_sub_path(head, root));
    }
}
