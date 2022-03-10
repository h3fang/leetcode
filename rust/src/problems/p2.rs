use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy.next;
        let mut h1 = l1.as_ref();
        let mut h2 = l2.as_ref();
        while h1.is_some() || h2.is_some() {
            let mut digit = carry;
            if let Some(node) = h1 {
                digit += node.val;
                h1 = node.next.as_ref();
            }

            if let Some(node) = h2 {
                digit += node.val;
                h2 = node.next.as_ref();
            }
            *tail = Some(Box::new(ListNode::new(digit % 10)));
            tail = &mut tail.as_mut().unwrap().next;
            carry = digit / 10;
        }
        if carry > 0 {
            *tail = Some(Box::new(ListNode::new(carry)));
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let l1 = ListNode::from_vec(&[2, 4, 3]);
        let l2 = ListNode::from_vec(&[5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        let expected = ListNode::from_vec(&[7, 0, 8]);
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let l1 = ListNode::from_vec(&[0]);
        let l2 = ListNode::from_vec(&[0]);
        let result = Solution::add_two_numbers(l1, l2);
        let expected = ListNode::from_vec(&[0]);
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let l1 = ListNode::from_vec(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(&[9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        let expected = ListNode::from_vec(&[8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(expected, result);
    }
}
