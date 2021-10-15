use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut part1 = None;
        let mut part2 = None;
        let mut h1 = &mut part1;
        let mut h2 = &mut part2;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                h1 = &mut h1.get_or_insert(node).next;
            } else {
                h2 = &mut h2.get_or_insert(node).next;
            }
        }
        *h1 = part2;
        part1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 4, 3, 2, 5, 2]);
        let expected = ListNode::from_vec(&[1, 2, 2, 4, 3, 5]);
        assert_eq!(
            expected.unwrap().to_string(),
            Solution::partition(head, 3).unwrap().to_string()
        );
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[2, 1]);
        let expected = ListNode::from_vec(&[1, 2]);
        assert_eq!(
            expected.unwrap().to_string(),
            Solution::partition(head, 2).unwrap().to_string()
        );
    }
}
