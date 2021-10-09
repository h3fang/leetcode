use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut to_remove = None;
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut current = &mut dummy.as_mut().unwrap().next;
        loop {
            match current {
                None => break,
                Some(n) if to_remove.is_some() && n.val == to_remove.unwrap() => {
                    *current = n.next.take();
                }
                Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => {
                    to_remove = Some(n.val);
                }
                Some(n) => {
                    current = &mut n.next;
                    to_remove = None;
                }
            }
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&vec![1, 2, 3, 3, 4, 4, 5]);
        let result = Solution::delete_duplicates(head);
        let expected = ListNode::from_vec(&[1, 2, 5]).unwrap().to_string();
        assert_eq!(expected, result.unwrap().to_string());
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&vec![1, 1, 1, 2, 3]);
        let result = Solution::delete_duplicates(head);
        let expected = ListNode::from_vec(&[2, 3]).unwrap().to_string();
        assert_eq!(expected, result.unwrap().to_string());
    }
}
