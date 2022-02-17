use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut is_even = true;
        while let Some(mut node) = head {
            is_even = !is_even;
            let next = node.next.take();
            if is_even {
                std::mem::swap(tail, &mut node);
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
            head = next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::linked_list::ListNode;

    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4]);
        let expected = ListNode::from_vec(&[2, 1, 4, 3]);
        assert_eq!(expected, Solution::swap_pairs(head));
    }
}
