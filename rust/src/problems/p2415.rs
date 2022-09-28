pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut q = VecDeque::new();
        q.push_back(root.clone());
        let mut level = 0;
        while !q.is_empty() {
            let num = q.len();
            if level % 2 == 1 {
                for i in 0..num / 2 {
                    std::mem::swap(
                        &mut q[i].as_ref().unwrap().borrow_mut().val,
                        &mut q[num - 1 - i].as_ref().unwrap().borrow_mut().val,
                    );
                }
            }

            if q[0].as_ref().unwrap().borrow().left.is_some() {
                for _ in 0..num {
                    let node = q.pop_front().unwrap().unwrap();
                    let n = node.borrow();
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                }
            } else {
                break;
            }
            level += 1;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[2, 3, 5, 8, 13, 21, 34]);
        let expected = TreeNode::from_vec(&[2, 5, 3, 8, 13, 21, 34]);
        assert_eq!(expected, Solution::reverse_odd_levels(root));
    }
}
