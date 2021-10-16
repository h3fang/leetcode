use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between_mem_n(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut nums = Vec::new();

        while let Some(node) = head {
            nums.push(node.val);
            head = node.next;
        }

        nums[left as usize - 1..=right as usize - 1].reverse();

        let mut next = None;

        while let Some(val) = nums.pop() {
            next = Some(Box::new(ListNode { val, next }));
        }

        next
    }

    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut h = &mut dummy;

        for _ in 1..left {
            h = &mut h.as_mut().unwrap().next;
        }

        let mut tail = h.as_mut().unwrap().next.take();

        for _ in left..right {
            let mut next = tail.as_mut().unwrap().next.take();
            tail.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = h.as_mut().unwrap().next.take();
            h.as_mut().unwrap().next = next;
        }

        while let Some(node) = h {
            h = &mut node.next;
        }

        *h = tail;

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::reverse_between(head, 2, 4);
        let expected = ListNode::from_vec(&[1, 4, 3, 2, 5]);
        assert_eq!(result.unwrap().to_string(), expected.unwrap().to_string());
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[5]);
        let result = Solution::reverse_between(head, 1, 1);
        let expected = ListNode::from_vec(&[5]);
        assert_eq!(result.unwrap().to_string(), expected.unwrap().to_string());
    }
}
