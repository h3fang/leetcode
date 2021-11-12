use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut h = &mut head;
        loop {
            match h {
                Some(n) if n.val == val => {
                    *h = n.next.take();
                }
                Some(n) => {
                    h = &mut n.next;
                }
                None => break,
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 6, 3, 4, 5, 6]);
        let val = 6;
        let result = Solution::remove_elements(head, val);
        let expected = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[7, 7, 7, 7]);
        let val = 7;
        let result = Solution::remove_elements(head, val);
        let expected = ListNode::from_vec(&[]);
        assert_eq!(expected, result);
    }
}
