use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let l1 = ListNode::from_vec(&[1, 2, 4]);
        let result = Solution::reverse_list(l1);
        let expected = ListNode::from_vec(&[4, 2, 1]).unwrap().to_string();
        assert_eq!(expected, result.unwrap().to_string());
    }
}
