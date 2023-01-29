use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Codec;

impl Codec {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut nodes: Vec<Option<i32>> = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(root);
        while let Some(n) = q.pop_front() {
            match n {
                Some(n) => {
                    let n = n.borrow();
                    nodes.push(Some(n.val));
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                }
                None => nodes.push(None),
            }
        }
        let mut nodes = nodes
            .iter()
            .map(|n| {
                n.map(|n| n.to_string())
                    .unwrap_or_else(|| "null".to_string())
            })
            .collect::<Vec<_>>();
        while !nodes.is_empty() {
            if nodes.last().unwrap() == "null" {
                nodes.pop();
            } else {
                break;
            }
        }
        format!("[{}]", nodes.join(","))
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let n = data.len();
        assert!(n >= 2);
        assert_eq!(&data[0..1], "[");
        assert_eq!(&data[n - 1..], "]");
        let nodes = data[1..n - 1]
            .split(',')
            .filter(|t| !t.is_empty())
            .map(|t| {
                if t == "null" {
                    None
                } else {
                    Some(
                        t.parse::<i32>()
                            .unwrap_or_else(|_| panic!("invalid node: {t}")),
                    )
                }
            })
            .collect::<Vec<_>>();
        let n = nodes.len();
        let root = nodes
            .first()
            .map(|e| e.map(|e| Rc::new(RefCell::new(TreeNode::new(e)))))
            .unwrap_or(None);
        let mut q = VecDeque::new();
        q.push_back(root.clone());
        let mut i = 1;
        while let Some(node) = q.pop_front() {
            if let Some(e) = node {
                if i == n {
                    break;
                }
                let mut e = e.borrow_mut();
                if let Some(v) = &nodes[i] {
                    e.left = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                    q.push_back(e.left.clone());
                }
                i += 1;
                if i == n {
                    break;
                }
                if let Some(v) = &nodes[i] {
                    e.right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                    q.push_back(e.right.clone());
                }
                i += 1;
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let c = Codec::new();
        let s = "[1,2,3,null,null,4,5]".to_string();
        let root = c.deserialize(s.to_string());
        assert_eq!(s, c.serialize(root));
    }

    #[test]
    fn case2() {
        let c = Codec::new();
        let s = "[]".to_string();
        let root = c.deserialize(s.to_string());
        assert_eq!(s, c.serialize(root));
    }
}
