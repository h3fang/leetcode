use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::linked_list::ListNode;
use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(
            head: &mut Option<&ListNode>,
            a: i32,
            b: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if a > b {
                None
            } else {
                let mid = a + (b - a + 1) / 2;
                let left = build_tree(head, a, mid - 1);
                let h = head.unwrap();
                let val = (*h).val;
                *head = h.next.as_deref();
                let right = build_tree(head, mid + 1, b);

                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
        }

        let mut n = 0;
        let mut h = head.as_ref();
        while let Some(node) = h {
            n += 1;
            h = node.next.as_ref();
        }
        let mut head = head.as_deref();
        build_tree(&mut head, 0, n - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[-10, -3, 0, 5, 9]);
        let expected = vec![
            TreeNode::from_vec(&[0, -3, 9, -10, null, 5]),
            TreeNode::from_vec(&[0, -10, 5, null, -3, null, 9]),
        ];
        let expected = expected
            .into_iter()
            .map(|t| t.unwrap().borrow().to_string())
            .collect::<Vec<_>>();
        let result = Solution::sorted_list_to_bst(head);
        let result = result.unwrap().borrow().to_string();
        println!("{:?}", expected);
        println!("{}", result);
        assert!(expected.contains(&result));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[6, 9]);
        let expected = vec![TreeNode::from_vec(&[9, 6]), TreeNode::from_vec(&[6, 9])];
        let expected = expected
            .into_iter()
            .map(|t| t.unwrap().borrow().to_string())
            .collect::<Vec<_>>();
        let result = Solution::sorted_list_to_bst(head);
        let result = result.unwrap().borrow().to_string();
        println!("{:?}", expected);
        println!("{}", result);
        assert!(expected.contains(&result));
    }
}
