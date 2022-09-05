pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut m: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((root, (0, 0)));
        let mut left = 0;
        let mut right = 0;
        while let Some((node, (i, j))) = q.pop_front() {
            if let Some(node) = node {
                let mut n = node.borrow_mut();
                m.entry(j).or_default().push((i, n.val));
                q.push_back((n.left.take(), (i + 1, j - 1)));
                q.push_back((n.right.take(), (i + 1, j + 1)));
                left = left.min(j - 1);
                right = right.max(j + 1);
            }
        }
        let mut result = vec![];
        for i in left..=right {
            if let Some(mut level) = m.remove(&i) {
                level.sort_unstable();
                result.push(level.into_iter().map(|e| e.1).collect());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 9, 20, null, null, 15, 7]);
        let expected = vec![vec![9], vec![3, 15], vec![20], vec![7]];
        assert_eq!(expected, Solution::vertical_traversal(root));
    }
}
