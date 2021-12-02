use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_odd = ListNode::new(0);
        let mut dummy_even = ListNode::new(0);
        let mut odd = &mut dummy_odd.next;
        let mut even = &mut dummy_even.next;
        let mut i = 1;
        while let Some(node) = &mut head {
            let next = node.next.take();
            if i % 2 == 0 {
                *even = Some(node.clone());
                even = &mut even.as_mut().unwrap().next;
            } else {
                *odd = Some(node.clone());
                odd = &mut odd.as_mut().unwrap().next;
            }
            head = next;
            i += 1;
        }
        *odd = dummy_even.next.take();
        dummy_odd.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(&[1, 3, 5, 2, 4]).unwrap().to_string();
        let result = Solution::odd_even_list(head).unwrap().to_string();
        assert_eq!(expected, result);
    }
}
