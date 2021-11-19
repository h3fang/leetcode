use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(
            root: Option<&Rc<RefCell<TreeNode>>>,
            p: i32,
            q: i32,
        ) -> (bool, bool, Option<Rc<RefCell<TreeNode>>>) {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let left = helper(n.left.as_ref(), p, q);
                    if left.2.is_some() {
                        return left;
                    }
                    let right = helper(n.right.as_ref(), p, q);
                    if right.2.is_some() {
                        return right;
                    }
                    let has_p = n.val == p || left.0 || right.0;
                    let has_q = n.val == q || left.1 || right.1;
                    (
                        has_p,
                        has_q,
                        if has_p && has_q { root.cloned() } else { None },
                    )
                }
                None => (false, false, None),
            }
        }
        helper(
            root.as_ref(),
            p.unwrap().borrow().val,
            q.unwrap().borrow().val,
        )
        .2
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::{find_node, null};

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
        let p = find_node(5, root.as_ref());
        let q = find_node(1, root.as_ref());
        assert_eq!(
            3,
            Solution::lowest_common_ancestor(root, p, q)
                .unwrap()
                .borrow()
                .val
        );
    }
}
