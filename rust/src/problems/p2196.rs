pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut m = HashMap::new();
        let mut s = descriptions.iter().map(|d| d[0]).collect::<HashSet<_>>();
        for d in descriptions {
            s.remove(&d[1]);
            let child = m
                .entry(d[1])
                .or_insert(Some(Rc::new(RefCell::new(TreeNode::new(d[1])))))
                .clone();
            let parent = m
                .entry(d[0])
                .or_insert(Some(Rc::new(RefCell::new(TreeNode::new(d[0])))));
            if d[2] == 1 {
                parent.as_mut().unwrap().borrow_mut().left = child;
            } else {
                parent.as_mut().unwrap().borrow_mut().right = child;
            }
        }
        let root = s.into_iter().next().unwrap();
        m[&root].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let descriptions = [
            [20, 15, 1],
            [20, 17, 0],
            [50, 20, 1],
            [50, 80, 0],
            [80, 19, 1],
        ]
        .iter()
        .map(|d| d.to_vec())
        .collect();
        let expected = TreeNode::from_vec(&[50, 20, 80, 15, 17, 19]);
        assert_eq!(expected, Solution::create_binary_tree(descriptions));
    }

    #[test]
    fn case2() {
        let descriptions = [[1, 2, 1], [2, 3, 0], [3, 4, 1]]
            .iter()
            .map(|d| d.to_vec())
            .collect();
        let expected = TreeNode::from_vec(&[1, 2, null, null, 3, 4]);
        assert_eq!(expected, Solution::create_binary_tree(descriptions));
    }
}
