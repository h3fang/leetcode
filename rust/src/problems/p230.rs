use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn kth_smallest_iterative(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack = Vec::new();
        loop {
            while let Some(node) = root {
                stack.push(Rc::clone(&node));
                root = node.borrow().left.clone();
            }
            let node = stack.pop().unwrap();
            k -= 1;
            if k == 0 {
                return node.borrow_mut().val;
            }
            root = node.borrow().right.clone();
        }
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn find(root: Option<&Rc<RefCell<TreeNode>>>, k: i32) -> (i32, i32) {
            match root {
                Some(node) => {
                    let (smaller, r) = find(node.borrow().left.as_ref(), k);
                    if smaller == k - 1 {
                        (k, node.borrow().val)
                    } else if smaller >= k {
                        (smaller, r)
                    } else {
                        let (bigger, r2) = find(node.borrow().right.as_ref(), k - 1 - smaller);
                        if bigger >= (k - 1 - smaller) {
                            (k, r2)
                        } else {
                            (smaller + 1 + bigger, -1)
                        }
                    }
                }
                None => (0, -1),
            }
        }
        find(root.as_ref(), k).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::null;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[3, 1, 4, null, 2]);
        assert_eq!(1, Solution::kth_smallest(root.clone(), 1));
        assert_eq!(1, Solution::kth_smallest_iterative(root, 1));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[5, 3, 6, 2, 4, null, null, 1]);
        assert_eq!(3, Solution::kth_smallest(root.clone(), 3));
        assert_eq!(3, Solution::kth_smallest_iterative(root, 3));
    }
}
