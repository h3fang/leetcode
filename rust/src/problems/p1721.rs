use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut fast = head.as_deref_mut().unwrap() as *mut ListNode;
        let mut slow = fast;
        unsafe {
            for _ in 1..k {
                fast = (*fast).next.as_deref_mut().unwrap();
            }
            let n1 = fast;
            while let Some(node) = &mut (*fast).next {
                fast = node.as_mut();
                slow = (*slow).next.as_deref_mut().unwrap();
            }
            std::mem::swap(&mut (*n1).val, &mut (*slow).val);
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(&[1, 4, 3, 2, 5]);
        assert_eq!(expected, Solution::swap_nodes(head, 2));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[7, 9, 6, 6, 7, 8, 3, 0, 9, 5]);
        let expected = ListNode::from_vec(&[7, 9, 6, 6, 8, 7, 3, 0, 9, 5]);
        assert_eq!(expected, Solution::swap_nodes(head, 5));
    }
}
