#![allow(non_upper_case_globals)]

use std::cell::RefCell;
use std::fmt;
use std::fmt::Write;
use std::rc::Rc;

pub const null: i32 = i32::MAX;

// Definition for a binary tree node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut i = 1;
        let root = nums
            .first()
            .map(|n| Rc::new(RefCell::new(TreeNode::new(*n))));
        let mut level = vec![root.clone()];
        while !level.is_empty() {
            let mut next_level = Vec::new();
            for node in level.into_iter().flatten() {
                let mut n = node.borrow_mut();
                if i == nums.len() {
                    break;
                }
                if nums[i] != null {
                    n.left = Some(Rc::new(RefCell::new(TreeNode::new(nums[i]))));
                    next_level.push(n.left.clone());
                }
                i += 1;
                if i == nums.len() {
                    break;
                }
                if nums[i] != null {
                    n.right = Some(Rc::new(RefCell::new(TreeNode::new(nums[i]))));
                    next_level.push(n.right.clone());
                }
                i += 1;
            }
            level = next_level;
        }
        root
    }

    pub fn inorder_traversal(&self) -> Vec<i32> {
        let mut r = Vec::new();
        let dummy = Some(Rc::new(RefCell::new((*self).clone())));
        let mut level = vec![dummy];
        while !level.is_empty() {
            let mut next_level = Vec::new();
            for node in level {
                match node {
                    Some(n) => {
                        let n = n.borrow();
                        r.push(n.val);
                        next_level.push(n.left.clone());
                        next_level.push(n.right.clone());
                    }
                    None => {
                        r.push(null);
                    }
                }
            }
            if next_level.iter().all(|n| n.is_none()) {
                break;
            }
            level = next_level;
        }
        while let Some(n) = r.last() {
            if *n == null {
                r.pop();
            } else {
                break;
            }
        }
        r
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nodes = self.inorder_traversal();
        let mut s = "[".to_string();
        for n in nodes {
            match n {
                null => s += "null,",
                x => {
                    let _ = write!(s, "{x},");
                }
            }
        }
        s.pop();
        s.push(']');
        write!(f, "{s}")
    }
}

#[allow(dead_code)]
pub fn find_node(val: i32, root: Option<&Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(n) => {
            let n = n.borrow();
            if n.val == val {
                root.cloned()
            } else {
                find_node(val, n.left.as_ref()).or_else(|| find_node(val, n.right.as_ref()))
            }
        }
        None => None,
    }
}

pub struct LeetCodeTreeNodes {
    pub nums: Vec<i32>,
}

impl fmt::Display for LeetCodeTreeNodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "[".to_string();
        for n in &self.nums {
            match *n {
                null => s += "null,",
                x => {
                    let _ = write!(s, "{x},");
                }
            }
        }
        s.pop();
        s.push(']');
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!("[1,2,3,4,5]", root.unwrap().borrow().to_string());
    }

    #[test]
    fn to_string2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));
        assert_eq!("[1,null,2,null,3]", root.unwrap().borrow().to_string());
    }

    #[test]
    fn to_string3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));
        assert_eq!("[1,null,2,3]", root.unwrap().borrow().to_string());
    }

    #[test]
    fn tree_from_vec1() {
        let nodes = LeetCodeTreeNodes {
            nums: vec![1, 2, 3, 4, 5],
        };
        let root = TreeNode::from_vec(&nodes.nums);
        assert_eq!(nodes.to_string(), root.unwrap().borrow().to_string());
    }

    #[test]
    fn tree_from_vec2() {
        let nodes = LeetCodeTreeNodes {
            nums: vec![2, 1, 3, null, 4, null, 7],
        };
        let root = TreeNode::from_vec(&nodes.nums);
        assert_eq!(nodes.to_string(), root.unwrap().borrow().to_string());
    }

    #[test]
    fn tree_from_vec3() {
        let nodes = LeetCodeTreeNodes {
            nums: vec![2, 1, 3, null, 4, null, 7, null, null, 5, null, null, 8],
        };
        let root = TreeNode::from_vec(&nodes.nums);
        assert_eq!(nodes.to_string(), root.unwrap().borrow().to_string());
    }

    #[test]
    fn tree_from_vec4() {
        let nodes = LeetCodeTreeNodes {
            nums: vec![1, null, 2, 3],
        };
        let root = TreeNode::from_vec(&nodes.nums);
        assert_eq!(nodes.to_string(), root.unwrap().borrow().to_string());
    }

    #[test]
    fn tree_from_vec5() {
        let nodes = LeetCodeTreeNodes {
            nums: vec![1, null, 2, null, 3],
        };
        let root = TreeNode::from_vec(&nodes.nums);
        assert_eq!(nodes.to_string(), root.unwrap().borrow().to_string());
    }
}
