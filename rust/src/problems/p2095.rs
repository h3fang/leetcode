use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n = 0;
        let mut h = head.as_ref();
        while let Some(node) = h {
            n += 1;
            h = node.next.as_ref();
        }
        n /= 2;
        let mut dummy = ListNode { val: 0, next: head };
        let mut prev = &mut dummy.next;
        while n > 0 {
            prev = &mut prev.as_mut().unwrap().next;
            n -= 1;
        }
        let next = prev.as_mut().unwrap().next.take();
        *prev = next;

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 3, 4, 7, 1, 2, 6]);
        let result = Solution::delete_middle(head).unwrap().to_string();
        let expected = ListNode::from_vec(&[1, 3, 4, 1, 2, 6]).unwrap().to_string();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 2, 3, 4]);
        let result = Solution::delete_middle(head).unwrap().to_string();
        let expected = ListNode::from_vec(&[1, 2, 4]).unwrap().to_string();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let head = ListNode::from_vec(&[2, 1]);
        let result = Solution::delete_middle(head).unwrap().to_string();
        let expected = ListNode::from_vec(&[2]).unwrap().to_string();
        assert_eq!(expected, result);
    }
}
