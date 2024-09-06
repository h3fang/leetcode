use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head as *const Option<Box<ListNode>>;
        let mut slow = &mut head;
        while let Some(node) = unsafe { (*fast).as_ref() } {
            match &node.next {
                Some(next) => {
                    fast = &next.next;
                    slow = &mut slow.as_mut().unwrap().next;
                }
                None => break,
            }
        }
        slow.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::middle_node(head);
        let expected = ListNode::from_vec(&[3, 4, 5]);
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5, 6]);
        let result = Solution::middle_node(head);
        let expected = ListNode::from_vec(&[4, 5, 6]);
        assert_eq!(expected, result);
    }
}
