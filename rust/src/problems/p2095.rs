use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast: *const Option<Box<ListNode>> = &head;
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut dummy;

        while let Some(n) = unsafe { (*fast).as_ref() }
            && let Some(nn) = &n.next
        {
            fast = &nn.next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        let mid = slow.as_mut().unwrap();
        let next = mid.next.take().unwrap().next.take();
        mid.next = next;
        dummy.unwrap().next
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
