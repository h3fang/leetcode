use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut i = 0;
        let mut h1 = &mut list1;
        while i + 1 < a {
            h1 = &mut h1.as_mut().unwrap().next;
            i += 1;
        }
        let mut mid = h1.as_mut().unwrap().next.take();
        while i < b {
            mid = mid.unwrap().next.take();
            i += 1;
        }
        let v1 = h1.as_ref().unwrap().val;
        *h1 = Some(Box::new(ListNode {
            val: v1,
            next: list2,
        }));
        while let Some(node) = h1 {
            h1 = &mut node.next;
        }
        *h1 = mid;
        list1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let list1 = ListNode::from_vec(&[0, 1, 2, 3, 4, 5]);
        let list2 = ListNode::from_vec(&[1000000, 1000001, 1000002]);
        let expected = ListNode::from_vec(&[0, 1, 2, 1000000, 1000001, 1000002, 5]);
        assert_eq!(expected, Solution::merge_in_between(list1, 3, 4, list2));
    }

    #[test]
    fn case2() {
        let list1 = ListNode::from_vec(&[2, 1, 2, 3, 4, 7, 6]);
        let list2 = ListNode::from_vec(&[1000000, 1000001, 1000002, 1000003, 1000004]);
        let expected = ListNode::from_vec(&[2, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6]);
        assert_eq!(expected, Solution::merge_in_between(list1, 2, 5, list2));
    }
}
