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
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    if p > n.val && q > n.val {
                        helper(n.right.as_ref(), p, q)
                    } else if p < n.val && q < n.val {
                        helper(n.left.as_ref(), p, q)
                    } else {
                        root.cloned()
                    }
                }
                None => None,
            }
        }
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        helper(root.as_ref(), p, q)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::{find_node, null};

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]);
        let p = find_node(2, root.as_ref());
        let q = find_node(8, root.as_ref());
        assert_eq!(
            6,
            Solution::lowest_common_ancestor(root, p, q)
                .unwrap()
                .borrow()
                .val
        );
    }
}
