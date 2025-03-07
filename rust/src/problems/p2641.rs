pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.inspect(|r| {
            r.borrow_mut().val = 0;
            let mut level = vec![r.clone()];
            while !level.is_empty() {
                let mut next = Vec::with_capacity(level.len() * 2);
                let sum = level
                    .iter()
                    .flat_map(|n| {
                        let l = n
                            .borrow()
                            .left
                            .as_ref()
                            .map(|l: &Rc<RefCell<TreeNode>>| l.borrow().val)
                            .unwrap_or(0);
                        let r = n
                            .borrow()
                            .right
                            .as_ref()
                            .map(|r| r.borrow().val)
                            .unwrap_or(0);
                        Some(l + r)
                    })
                    .sum::<i32>();
                for n in level {
                    let mut n = n.borrow_mut();
                    let lr = n.left.as_ref().map(|n| n.borrow().val).unwrap_or(0)
                        + n.right.as_ref().map(|n| n.borrow().val).unwrap_or(0);
                    if let Some(l) = &mut n.left {
                        l.borrow_mut().val = sum - lr;
                        next.push(l.clone());
                    }
                    if let Some(r) = &mut n.right {
                        r.borrow_mut().val = sum - lr;
                        next.push(r.clone());
                    }
                }
                level = next;
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 4, 9, 1, 10, null, 7]);
        let expected = TreeNode::from_vec(&[0, 0, 0, 7, 7, null, 11]);
        assert_eq!(expected, Solution::replace_value_in_tree(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[3, 1, 2]);
        let expected = TreeNode::from_vec(&[0, 0, 0]);
        assert_eq!(expected, Solution::replace_value_in_tree(root));
    }
}
