pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::VecDeque;

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut nodes = vec![None; 10_0001];
        let mut parent = [0; 10_0001];
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            nodes: &mut [Option<Rc<RefCell<TreeNode>>>],
            parent: &mut [i32],
        ) {
            if let Some(node) = root {
                let n = node.borrow_mut();
                let v = n.val;
                nodes[v as usize] = Some(node.clone());
                if let Some(left) = n.left.clone() {
                    parent[left.borrow().val as usize] = v;
                    dfs(Some(left), nodes, parent);
                }
                if let Some(right) = n.right.clone() {
                    parent[right.borrow().val as usize] = v;
                    dfs(Some(right), nodes, parent);
                }
            }
        }
        dfs(root, &mut nodes, &mut parent);
        let mut result = -1;
        let mut q = VecDeque::new();
        q.push_back(nodes[start as usize].take());
        while !q.is_empty() {
            let n = q.len();
            for _ in 0..n {
                let node = q.pop_front().unwrap().unwrap();
                let v = node.borrow().val;
                let p = parent[v as usize];
                if let Some(p) = nodes[p as usize].take() {
                    q.push_back(Some(p));
                }
                let left = node.borrow_mut().left.take();
                if let Some(left) = left {
                    let v = left.borrow().val;
                    if let Some(p) = nodes[v as usize].take() {
                        q.push_back(Some(p));
                    }
                }

                let right = node.borrow_mut().right.take();
                if let Some(right) = right {
                    let v = right.borrow().val;
                    if let Some(p) = nodes[v as usize].take() {
                        q.push_back(Some(p));
                    }
                }
            }
            result += 1;
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
        let root = TreeNode::from_vec(&[1, 5, 3, null, 4, 10, 6, 9, 2]);
        assert_eq!(4, Solution::amount_of_time(root, 3));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1]);
        assert_eq!(0, Solution::amount_of_time(root, 1));
    }
}
