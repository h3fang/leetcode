use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                {
                    let mut n = node.borrow_mut();
                    match n.val.cmp(&key) {
                        std::cmp::Ordering::Less => {
                            n.right = Self::delete_node(n.right.take(), key)
                        }
                        std::cmp::Ordering::Equal => match (n.left.take(), n.right.take()) {
                            (None, None) => return None,
                            (None, Some(right)) => return Some(right),
                            (Some(left), None) => return Some(left),
                            (Some(left), Some(right)) => {
                                let mut h = right.clone();
                                while h.borrow().left.is_some() {
                                    let left = h.borrow().left.as_ref().unwrap().clone();
                                    h = left;
                                }
                                h.borrow_mut().left = Some(left);
                                return Some(right);
                            }
                        },
                        std::cmp::Ordering::Greater => {
                            n.left = Self::delete_node(n.left.take(), key)
                        }
                    }
                }
                Some(node)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 3, 6, 2, 4, null, 7]);
        let key = 3;
        let expected1 = TreeNode::from_vec(&[5, 4, 6, 2, null, null, 7])
            .unwrap()
            .borrow()
            .to_string();
        let expected2 = TreeNode::from_vec(&[5, 2, 6, null, 4, null, 7])
            .unwrap()
            .borrow()
            .to_string();
        let result = Solution::delete_node(root, key)
            .unwrap()
            .borrow()
            .to_string();
        println!("{result}");
        println!("{expected1}");
        println!("{expected2}");
        assert!(result == expected1 || result == expected2);
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 3, 6, 2, 4, null, 7]);
        let key = 5;
        let expected1 = TreeNode::from_vec(&[6, 3, 7, 2, 4])
            .unwrap()
            .borrow()
            .to_string();
        let result = Solution::delete_node(root, key)
            .unwrap()
            .borrow()
            .to_string();
        println!("{result}");
        println!("{expected1}");
        assert!(result == expected1);
    }
}
