use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        fn mid(head: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
            let mut fast = head as *const Option<Box<ListNode>>;
            let mut slow = head;
            while let Some(node) = unsafe { (*fast).as_ref() } {
                match &node.next {
                    Some(nn) => {
                        fast = &nn.next;
                        slow = &mut slow.as_mut().unwrap().next;
                    }
                    None => break,
                }
            }
            slow
        }

        fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut h) = head {
                let next = h.next;
                h.next = prev;
                prev = Some(h);
                head = next;
            }
            prev
        }

        fn merge(
            mut left: Option<Box<ListNode>>,
            mut right: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut dummy = Some(Box::new(ListNode::new(0)));
            let mut tail = &mut dummy;
            while left.is_some() && right.is_some() {
                let l_next = left.as_mut().unwrap().next.take();
                let r_next = right.as_mut().unwrap().next.take();

                tail.as_mut().unwrap().next = left;
                tail = &mut tail.as_mut().unwrap().next;
                tail.as_mut().unwrap().next = right;
                tail = &mut tail.as_mut().unwrap().next;

                left = l_next;
                right = r_next;
            }

            if left.is_some() {
                tail.as_mut().unwrap().next = left;
            } else if right.is_some() {
                tail.as_mut().unwrap().next = right;
            }

            dummy.unwrap().next
        }

        if head.is_none() {
            return;
        }

        let m = mid(head);
        let right = reverse(m.take());
        let h = merge(head.take(), right);
        *head = h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut head = ListNode::from_vec(&[1, 2, 3, 4]);
        let expected = ListNode::from_vec(&[1, 4, 2, 3]);
        Solution::reorder_list(&mut head);
        assert_eq!(expected, head);
    }

    #[test]
    fn case2() {
        let mut head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(&[1, 5, 2, 4, 3]);
        Solution::reorder_list(&mut head);
        assert_eq!(expected, head);
    }
}
