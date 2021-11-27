use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr = head;

        loop {
            if curr.is_none() {
                break;
            } else {
                let mut left = dummy_head.as_mut();
                while left.as_ref().unwrap().next.is_some()
                    && left.as_ref().unwrap().next.as_ref().unwrap().val
                        < curr.as_ref().unwrap().val
                {
                    left = left.unwrap().next.as_mut();
                }
                let right = left.as_mut().unwrap().next.take();
                let next = curr.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = right;
                left.as_mut().unwrap().next = curr;
                curr = next;
            }
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[4, 2, 1, 3]);
        let expected = ListNode::from_vec(&[1, 2, 3, 4]);
        assert_eq!(expected, Solution::insertion_sort_list(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[-1, 5, 3, 4, 0]);
        let expected = ListNode::from_vec(&[-1, 0, 3, 4, 5]);
        assert_eq!(expected, Solution::insertion_sort_list(head));
    }
}
