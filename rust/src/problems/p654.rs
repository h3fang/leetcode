pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut s: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        for e in nums {
            let node = Rc::new(RefCell::new(TreeNode::new(e)));
            while !s.is_empty() && e > s.last().unwrap().as_ref().unwrap().borrow().val {
                node.borrow_mut().left = s.pop().unwrap();
            }
            if !s.is_empty() {
                s.last_mut().unwrap().as_mut().unwrap().borrow_mut().right = Some(node.clone());
            }
            s.push(Some(node));
        }
        s[0].take()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let result = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
        let expected = TreeNode::from_vec(&[6, 3, 5, null, 2, 0, null, null, 1]);
        assert_eq!(expected, result);
    }
}
