use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut arr = vec![];
        fn postorder(root: Option<&Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
            if let Some(node) = root {
                postorder(node.borrow().left.as_ref(), arr);
                postorder(node.borrow().right.as_ref(), arr);
                arr.push(node.borrow().val);
            }
        }
        postorder(root.as_ref(), &mut arr);
        let s = arr.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        s.join(",")
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums = data
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        fn construct_bst(nums: &mut Vec<i32>, lb: i32, ub: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(&n) = nums.last() {
                if n < lb || n > ub {
                    return None;
                }
                nums.pop();
                let mut node = TreeNode::new(n);
                node.right = construct_bst(nums, n, ub);
                node.left = construct_bst(nums, lb, n);
                return Some(Rc::new(RefCell::new(node)));
            }
            None
        }
        construct_bst(&mut nums, i32::MIN, i32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::TreeNode;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        let c = Codec::new();
        let s = c.serialize(root.clone());
        let tree = c.deserialize(s);
        assert_eq!(root, tree);
    }
}
