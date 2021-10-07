use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut level = vec![root];
        while !level.is_empty() {
            let mut next_level = Vec::new();
            for (i, node) in level.iter().enumerate() {
                if let Some(n) = node {
                    let left = n.borrow_mut().left.take();
                    let right = n.borrow_mut().right.take();
                    next_level.push(left);
                    next_level.push(right);
                } else {
                    return level[i + 1..].iter().all(|n| n.is_none())
                        && next_level.iter().all(|n| n.is_none());
                }
            }
            level = next_level;
        }
        true
    }
}
